//! Test helper for unit tests

use axum::{Extension, Router};
use book_api::{
    config::logger,
    layers::{self, MakeRequestUuid},
    routes,
};
use rand::distr::{Alphanumeric, SampleString};
use sqlx::{Connection, PgConnection, PgPool, Postgres, postgres::PgPoolOptions};
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::ServiceBuilderExt;

pub struct TestApp {
    pub router: Router,
    pub _database: TestDatabase,
}

pub struct TestAppBuilder {
    router: Router,
    database: TestDatabase,
}

impl TestAppBuilder {
    pub async fn new() -> Self {
        let db = TestDatabase::new().await;

        let mut router = Router::new().nest("/api/v1/book", routes::api());
        router = router.merge(routes::web());
        router = router.layer(Extension(db.database().await));

        Self {
            router,
            database: db,
        }
    }

    #[allow(unused)]
    pub fn with_logger(self) -> Self {
        logger::init("test").unwrap();
        let layers = ServiceBuilder::new()
            .set_x_request_id(MakeRequestUuid)
            .layer(layers::logger::LoggerLayer)
            .into_inner();

        Self {
            router: self.router.layer(layers),
            database: self.database,
        }
    }

    pub fn build(self) -> TestApp {
        TestApp {
            router: self.router,
            _database: self.database,
        }
    }
}

#[derive(Debug)]
pub struct TestDatabase {
    url: String,
    pool: PgPool,
}

/// Sets up a new DB for running tests with.
impl TestDatabase {
    pub async fn new() -> Self {
        let db_url = Self::url();

        Self::create_database(&db_url).await;
        Self::run_migrations(&db_url).await;

        let pool = PgPool::connect(&db_url).await.unwrap();

        Self { url: db_url, pool }
    }

    pub async fn database(&self) -> PgPool {
        self.pool.clone()
    }

    /// Drop database after the test
    pub async fn drop_database(&self) {
        let (conn, db_name) = Self::parse_url(&self.url);

        let pool = PgPoolOptions::new()
            .max_connections(1)
            .min_connections(1)
            .max_lifetime(Some(Duration::from_secs(5)))
            .acquire_timeout(Duration::from_secs(5))
            .idle_timeout(Duration::from_secs(5))
            .test_before_acquire(false)
            .connect(conn)
            .await
            .expect("failed to create PostgreSQL pool creation");

        let sql = format!(
            r#"
            SELECT *, pg_terminate_backend(pid) FROM pg_stat_activity
            WHERE pid <> pg_backend_pid() AND datname = '{}';"#,
            &db_name
        );
        sqlx::query::<Postgres>(&sql)
            .execute(&pool)
            .await
            .expect("failed to terminate open database connections");

        let sql = format!(r#"DROP DATABASE "{}""#, &db_name);
        sqlx::query::<Postgres>(&sql)
            .execute(&pool)
            .await
            .expect("failed to drop database");
    }

    /// Generate url with a random database name
    fn url() -> String {
        dotenvy::dotenv().ok();

        // Set up the database per tests
        let suffix: String = Alphanumeric.sample_string(&mut rand::rng(), 16);
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL missing from environment");

        format!("{}_{}", db_url, suffix)
    }

    /// Parse database URL and return the database name in a separate variable
    fn parse_url(url: &str) -> (&str, &str) {
        let separator_pos = url.rfind('/').unwrap();
        let conn = &url[..=separator_pos - 1];
        let name = &url[separator_pos + 1..];

        (conn, name)
    }

    /// Create the test database
    async fn create_database(url: &str) {
        let (conn, db_name) = Self::parse_url(url);

        let mut pool = PgConnection::connect(conn).await.unwrap();

        let sql = format!(r#"CREATE DATABASE "{}";"#, &db_name);
        sqlx::query::<Postgres>(&sql)
            .execute(&mut pool)
            .await
            .unwrap();
    }

    /// Launch migrations
    async fn run_migrations(url: &str) {
        let (conn, db_name) = Self::parse_url(url);
        let mut pool = PgConnection::connect(&format!("{}/{}", conn, db_name))
            .await
            .unwrap();

        sqlx::migrate!("./migrations")
            .run(&mut pool)
            .await
            .expect("failed to perform database migrations");
    }
}

impl Drop for TestDatabase {
    fn drop(&mut self) {
        // Drop the DB Pool
        std::thread::scope(|s| {
            s.spawn(|| {
                let runtime = tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()
                    .unwrap();
                runtime.block_on(self.drop_database());
            });
        });
    }
}
