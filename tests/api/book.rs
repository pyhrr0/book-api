use super::helpers::book::{TestBook, create, delete, fetch_all, fetch_one, update};
use crate::{
    api::helpers::TestPaginateResponse,
    helper::{TestApp, TestAppBuilder},
};
use axum::http::StatusCode;
use uuid::Uuid;

#[tokio::test]
async fn test_api_create_book() {
    let app: TestApp = TestAppBuilder::new().await.build();

    let response = create(
        &app,
        serde_json::json!({
            "title": "foo",
            "author": "bar",
        })
        .to_string(),
    )
    .await;
    assert_eq!(response.status_code, StatusCode::OK);
}

#[tokio::test]
async fn test_api_create_book_invalid_json() {
    let app: TestApp = TestAppBuilder::new().await.build();

    let response = create(&app, "invalid_json".to_string()).await;
    assert_eq!(response.status_code, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_api_fetch_all_books() {
    let app: TestApp = TestAppBuilder::new().await.build();

    for i in 0..2 {
        let f = create(
            &app,
            serde_json::json!({
                "title": format!("foo-{i}"),
                "author": "bar",
            })
            .to_string(),
        )
        .await;
        println!("{:?}", f);
    }

    let response = fetch_all(&app, None).await;
    assert_eq!(response.status_code, StatusCode::OK);

    let activities: TestPaginateResponse<Vec<TestBook>> =
        serde_json::from_str(&response.body.to_string()).expect("failed to deserialise body");
    assert_eq!(activities.data.len(), 2);
    assert_eq!(activities.total, 2);
}

#[tokio::test]
async fn test_api_fetch_all_books_invalid_filter() {
    let app: TestApp = TestAppBuilder::new().await.build();

    let response = fetch_all(&app, Some("p=not_an_int")).await;
    assert_eq!(response.status_code, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_api_fetch_one_book() {
    let app: TestApp = TestAppBuilder::new().await.build();

    let response = create(
        &app,
        serde_json::json!({
            "title": "foo",
            "author": "bar",
        })
        .to_string(),
    )
    .await;

    let book_id = TestBook::from_body(&response.body.to_string()).id;

    let response = fetch_one(&app, &book_id).await;
    let body = TestBook::from_body(&response.body.to_string());
    assert_eq!(response.status_code, StatusCode::OK);
    assert_eq!(body.id, book_id);
    assert_eq!(body.updated_at, None);
}

#[tokio::test]
async fn test_api_fetch_one_book_invalid_id() {
    let app: TestApp = TestAppBuilder::new().await.build();

    let response = fetch_one(&app, "not_an_uuid").await;
    assert_eq!(response.status_code, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_api_fetch_one_book_unknown_id() {
    let app: TestApp = TestAppBuilder::new().await.build();

    let response = fetch_one(&app, &Uuid::new_v4().to_string()).await;
    assert_eq!(response.status_code, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_api_update_book() {
    let app: TestApp = TestAppBuilder::new().await.build();

    let response = create(
        &app,
        serde_json::json!({
            "title": "foo",
            "author": "bar",
        })
        .to_string(),
    )
    .await;

    let book_id = TestBook::from_body(&response.body.to_string()).id;

    let response = update(
        &app,
        serde_json::json!({
            "title": "bar",
            "author": "foo",
        })
        .to_string(),
        &book_id,
    )
    .await;
    assert_eq!(response.status_code, StatusCode::OK);

    let book = TestBook::from_body(&response.body.to_string());
    assert_eq!(book.title, String::from("bar"));
    assert_eq!(book.author, String::from("foo"));
    assert_ne!(book.updated_at, None)
}

#[tokio::test]
async fn test_api_update_book_unknown_id() {
    let app: TestApp = TestAppBuilder::new().await.build();

    let response = update(
        &app,
        serde_json::json!({
            "title": "bar",
            "author": "foo",
        })
        .to_string(),
        &Uuid::new_v4().to_string(),
    )
    .await;
    assert_eq!(response.status_code, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_api_delete_book() {
    let app: TestApp = TestAppBuilder::new().await.build();

    let response = create(
        &app,
        serde_json::json!({
            "title": "foo",
            "author": "bar",
        })
        .to_string(),
    )
    .await;

    let book_id = TestBook::from_body(&response.body.to_string()).id;

    let response = delete(&app, &book_id).await;
    assert_eq!(response.status_code, StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn test_api_delete_book_invalid_id() {
    let app: TestApp = TestAppBuilder::new().await.build();
    let response = delete(&app, "not_an_uuid").await;
    assert_eq!(response.status_code, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_api_delete_book_unknown_id() {
    let app: TestApp = TestAppBuilder::new().await.build();
    let response = delete(&app, &Uuid::new_v4().to_string()).await;
    assert_eq!(response.status_code, StatusCode::INTERNAL_SERVER_ERROR);
}
