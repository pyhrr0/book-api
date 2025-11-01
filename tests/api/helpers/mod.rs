pub mod book;

use crate::helper::TestApp;
use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use tower::ServiceExt;

/// HTTP response for test
#[derive(Debug)]
pub struct TestResponse {
    pub status_code: StatusCode,
    pub body: Value,
    pub _headers: HashMap<String, String>,
}

impl TestResponse {
    /// Create a new `TestResponse`
    pub async fn new(app: &TestApp, url: &str, method: &str, body: Option<String>) -> Self {
        let request = Request::builder()
            .uri(url)
            .method(method)
            .header("Content-Type", "application/json");

        let request = request.body(match body {
            None => Body::empty(),
            Some(body) => body.into(),
        });

        let response = app.router.clone().oneshot(request.unwrap()).await.unwrap();

        let status_code = response.status();
        let body = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("failed to convert body into bytes");
        let body: Value = serde_json::from_slice(&body).unwrap_or(Value::Null);

        TestResponse {
            status_code,
            body,
            _headers: HashMap::new(),
        }
    }
}

#[derive(Deserialize)]
pub struct TestPaginateResponse<T> {
    pub data: T,
    pub total: i64,
}
