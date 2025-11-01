use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
    pub id: String,
    pub title: String,
    pub author: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl Book {
    pub fn new(book: BookCreation) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            title: book.title,
            author: book.author,
            created_at: Utc::now(),
            updated_at: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct BookCreation {
    pub title: String,
    pub author: String,
}
