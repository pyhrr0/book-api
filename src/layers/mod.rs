pub mod logger;
pub mod prometheus;

use crate::app_error;
use crate::types::{AppError, AppErrorCode};
use axum::{
    body::{Body, to_bytes},
    http::{HeaderValue, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::str::from_utf8;
use tower_http::request_id::{MakeRequestId, RequestId};
use uuid::Uuid;

#[derive(Clone, Copy)]
pub struct MakeRequestUuid;

impl MakeRequestId for MakeRequestUuid {
    fn make_request_id<B>(&mut self, _request: &Request<B>) -> Option<RequestId> {
        let id = Uuid::new_v4().to_string().parse();
        match id {
            Ok(id) => Some(RequestId::new(id)),
            _ => None,
        }
    }
}

// =============== Override some HTTP errors ================

/// Layer which override some HTTP errors by using `AppError`
pub async fn override_http_errors(req: Request<Body>, next: Next) -> impl IntoResponse {
    let response = next.run(req).await;

    // If it is an image, audio or video, we return response
    let headers = response.headers();
    if let Some(content_type) = headers.get("content-type") {
        let content_type = content_type.to_str().unwrap_or_default();
        if content_type.starts_with("image/")
            || content_type.starts_with("audio/")
            || content_type.starts_with("video/")
        {
            return response;
        }
    }

    let (parts, body) = response.into_parts();
    match to_bytes(body, usize::MAX).await {
        Ok(body_bytes) => match String::from_utf8(body_bytes.to_vec()) {
            Ok(body) => match parts.status {
                StatusCode::METHOD_NOT_ALLOWED => {
                    app_error!(AppErrorCode::MethodNotAllowed).into_response()
                }
                StatusCode::UNPROCESSABLE_ENTITY => {
                    if body.contains("Failed to deserialize the JSON body") {
                        app_error!(AppErrorCode::UnprocessableEntity, body).into_response()
                    } else {
                        Response::from_parts(parts, Body::from(body))
                    }
                }
                _ => Response::from_parts(parts, Body::from(body)),
            },
            Err(err) => app_error!(AppErrorCode::InternalError, err.to_string()).into_response(),
        },
        Err(err) => app_error!(AppErrorCode::InternalError, err.to_string()).into_response(),
    }
}

// =============== Utils ================

/// Convert `HeaderValue` to `&str`
pub fn header_value_to_str(value: Option<&HeaderValue>) -> &str {
    match value {
        Some(value) => from_utf8(value.as_bytes()).unwrap_or_default(),
        None => "",
    }
}
