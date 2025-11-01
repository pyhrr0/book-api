//! Helpers for user API tests

use super::TestResponse;
use crate::helper::TestApp;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TestBook {
    pub id: String,
    pub title: String,
    pub author: String,
    pub _created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl TestBook {
    pub fn from_body(body: &str) -> Self {
        serde_json::from_str(body).expect("an error occurred when deserialising user body")
    }
}

/// User creation request helper
pub async fn create(app: &TestApp, body: String) -> TestResponse {
    TestResponse::new(app, "/api/v1/book", "POST", Some(body)).await
}

/// Return all processing activities
pub async fn fetch_all(app: &TestApp, params: Option<&str>) -> TestResponse {
    TestResponse::new(
        app,
        &format!("/api/v1/book?{}", params.unwrap_or_default(),),
        "GET",
        None,
    )
    .await
}

/// Return a processing book
pub async fn fetch_one(app: &TestApp, id: &str) -> TestResponse {
    TestResponse::new(app, &format!("/api/v1/book/{id}"), "GET", None).await
}

/// Update a processing book
pub async fn update(app: &TestApp, body: String, id: &str) -> TestResponse {
    TestResponse::new(app, &format!("/api/v1/book/{id}"), "PUT", Some(body)).await
}

/// Delete a processing book
pub async fn delete(app: &TestApp, id: &str) -> TestResponse {
    TestResponse::new(app, &format!("/api/v1/book/{id}"), "DELETE", None).await
}
