use crate::config::Config;
use crate::types::{CliError, CliResult};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::time::Duration;

pub async fn init_db_pool(settings: &Config) -> CliResult<Pool<Postgres>> {
    let url = &settings.database_url;
    let max_connections = settings.database_max_connections;
    let min_connections = settings.database_min_connections;
    let connection_lifetime = settings.database_connection_lifetime;
    let connect_timeout = settings.database_connect_timeout;
    let idle_timeout = settings.database_idle_timeout;

    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .min_connections(min_connections)
        .max_lifetime(Some(Duration::from_secs(connection_lifetime)))
        .acquire_timeout(Duration::from_secs(connect_timeout))
        .idle_timeout(Duration::from_secs(idle_timeout))
        .test_before_acquire(true)
        .connect(url)
        .await
        .map_err(|err| CliError::DatabaseError(err.to_string()))?;

    if settings.database_auto_migration {
        info!("Run database migrations");
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .map_err(|err| {
                CliError::DatabaseError(format!("failed to run database migrations: {err}"))
            })?
    }

    Ok(pool)
}
