use crate::handlers;
use axum::{
    Router,
    response::Redirect,
    routing::{delete, get, post, put},
};

pub fn web() -> Router<()> {
    Router::new()
        .route(
            "/",
            get(|| async { Redirect::permanent("/rapidoc-ui.html") }),
        )
        .route("/health-check", get(handlers::web::health_check))
}

pub fn api() -> Router<()> {
    Router::new()
        .route("/", post(handlers::book::create))
        .route("/", get(handlers::book::get_all))
        .route("/{id}", get(handlers::book::get_by_id))
        .route("/{id}", put(handlers::book::update))
        .route("/{id}", delete(handlers::book::delete))
}
