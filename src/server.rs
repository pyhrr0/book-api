use crate::{
    config::{Config, databases, logger},
    layers::{self, MakeRequestUuid, prometheus::PrometheusMetric},
    routes,
};
use axum::{Extension, Router, middleware, routing::get};
use color_eyre::Result;
use std::future::ready;
use std::net::SocketAddr;
use tokio::signal;
use tower::ServiceBuilder;
use tower_http::{ServiceBuilderExt, services::ServeDir};

/// Starts API server
pub async fn start_server() -> Result<()> {
    color_eyre::install()?;

    let settings = Config::from_env()?;
    let app = get_app(&settings).await?;

    let addr = format!("{}:{}", settings.bind_address, settings.bind_port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("Starting server on {}...", &addr);

    let server = axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    );

    // No graceful shutdown in development environment
    if &settings.environment == "development" {
        Ok(server.await?)
    } else {
        Ok(server.with_graceful_shutdown(shutdown_signal()).await?)
    }
}

pub async fn get_app(settings: &Config) -> Result<Router> {
    logger::init(&settings.environment)?;

    let pool = databases::init_db_pool(settings).await?;

    let layers = ServiceBuilder::new()
        .set_x_request_id(MakeRequestUuid)
        .layer(layers::logger::LoggerLayer)
        .propagate_x_request_id();

    let mut app = Router::new().nest("/api/v1/book", routes::api());

    app = app.merge(routes::web());

    if settings.prometheus_metrics_enabled {
        let handle = PrometheusMetric::get_handle()?;
        app = app
            .nest(
                "/metrics",
                Router::new().route("/", get(move || ready(handle.render()))),
            )
            .route_layer(middleware::from_fn(PrometheusMetric::get_layer));
    }

    app = app
        .fallback_service(ServeDir::new("assets").append_index_html_on_directories(true))
        .layer(middleware::from_fn(layers::override_http_errors))
        .layer(Extension(pool))
        .layer(layers);

    Ok(app)
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("signal received, starting graceful shutdown");
}
