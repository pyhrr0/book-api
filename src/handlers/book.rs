use crate::{
    app_error,
    models::book::{Book, BookCreation},
    repositories::book::BookRepository,
    types::{AppError, AppErrorCode, AppResult},
    utils::{
        extractors::{ExtractRequestId, Path, Query},
        query::{PaginateResponse, PaginateSort, PaginateSortQuery},
        validation::validate_request_data,
    },
};
use axum::{
    extract::{Extension, Json},
    http::StatusCode,
};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

// Route: POST /api/v1/book
#[instrument(skip(pool))]
pub async fn create(
    Extension(pool): Extension<Pool<Postgres>>,
    ExtractRequestId(request_id): ExtractRequestId,
    Json(payload): Json<BookCreation>,
) -> AppResult<Json<Book>> {
    validate_request_data(&payload)?;

    let mut book = Book::new(payload);
    BookRepository::create(&pool, &mut book).await?;

    Ok(Json(book))
}

// Route: GET /api/v1/book
#[instrument(skip(pool))]
pub async fn get_all(
    Query(pagination): Query<PaginateSortQuery>,
    Extension(pool): Extension<Pool<Postgres>>,
    ExtractRequestId(request_id): ExtractRequestId,
) -> AppResult<Json<PaginateResponse<Vec<Book>>>> {
    let paginate_sort = PaginateSort::from(pagination);
    let books = BookRepository::get_all(&pool, &paginate_sort).await?;

    Ok(Json(books))
}

// Route: GET "/api/v1/book/:id"
#[instrument(skip(pool))]
pub async fn get_by_id(
    Path(id): Path<Uuid>,
    Extension(pool): Extension<Pool<Postgres>>,
    ExtractRequestId(request_id): ExtractRequestId,
) -> AppResult<Json<Book>> {
    let book = BookRepository::get_by_id(&pool, id.to_string()).await?;
    match book {
        Some(book) => Ok(Json(book)),
        _ => Err(app_error!(
            AppErrorCode::NotFound,
            "book could not be found"
        )),
    }
}

// Route: PUT "/api/v1/book/:id"
#[instrument(skip(pool))]
pub async fn update(
    Path(id): Path<Uuid>,
    Extension(pool): Extension<Pool<Postgres>>,
    ExtractRequestId(request_id): ExtractRequestId,
    Json(payload): Json<BookCreation>,
) -> AppResult<Json<Book>> {
    validate_request_data(&payload)?;

    BookRepository::update(&pool, id.to_string(), &payload).await?;

    let book = BookRepository::get_by_id(&pool, id.to_string()).await?;
    match book {
        Some(book) => Ok(Json(book)),
        _ => Err(app_error!(
            AppErrorCode::NotFound,
            "book could not be found"
        )),
    }
}

// Route: DELETE "/api/v1/book/:id"
#[instrument(skip(pool))]
pub async fn delete(
    Path(id): Path<Uuid>,
    Extension(pool): Extension<Pool<Postgres>>,
    ExtractRequestId(request_id): ExtractRequestId,
) -> AppResult<StatusCode> {
    let result = BookRepository::delete(&pool, id.to_string()).await?;
    match result {
        1 => Ok(StatusCode::NO_CONTENT),
        _ => Err(app_error!(
            AppErrorCode::InternalError,
            "no book or book already deleted"
        )),
    }
}
