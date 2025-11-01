use crate::{
    models::book::{Book, BookCreation},
    types::AppResult,
    utils::query::{PaginateResponse, PaginateSort},
};
use chrono::Utc;
use futures::TryStreamExt;
use sqlx::{PgPool, Row};

pub struct BookRepository;

impl BookRepository {
    /// Add a new book
    #[tracing::instrument(skip(pool))]
    pub async fn create(pool: &PgPool, book: &mut Book) -> AppResult<()> {
        sqlx::query!(
            r#"
                INSERT INTO book (id, title, author, created_at)
                VALUES ( $1, $2, $3, $4)
            "#,
            book.id,
            book.title,
            book.author,
            book.created_at,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Returns all books
    #[instrument(skip(pool))]
    pub async fn get_all<'a>(
        pool: &'a PgPool,
        paginate_sort: &'a PaginateSort,
    ) -> AppResult<PaginateResponse<Vec<Book>>> {
        let total = Self::get_total(pool).await?;

        let mut query = String::from(
            "
            SELECT id, title, author, created_at, updated_at
            FROM book
            ",
        );

        // Sorts and pagination
        query.push_str(&paginate_sort.get_sorts_sql(Some(&[
            "id",
            "title",
            "author",
            "created_at",
            "updated_at",
        ])));
        query.push_str(&paginate_sort.get_pagination_sql());

        let mut rows = sqlx::query(&query)
            .bind(i32::try_from(paginate_sort.limit)?)
            .bind(i32::try_from(paginate_sort.offset)?)
            .fetch(pool);

        let mut books = vec![];
        while let Some(row) = rows.try_next().await? {
            books.push(Book {
                id: row.try_get("id")?,
                title: row.try_get("title")?,
                author: row.try_get("author")?,
                created_at: row.try_get("created_at")?,
                updated_at: row.try_get("updated_at")?,
            });
        }
        Ok(PaginateResponse { data: books, total })
    }

    /// Returns a book by its ID
    #[instrument(skip(pool))]
    pub async fn get_by_id(pool: &PgPool, id: String) -> AppResult<Option<Book>> {
        let result = sqlx::query!(
            r#"
                SELECT *
                FROM book
                WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        match result {
            Some(result) => Ok(Some(Book {
                id: result.id,
                title: result.title,
                author: result.author,
                created_at: result.created_at,
                updated_at: result.updated_at,
            })),
            None => Ok(None),
        }
    }

    /// Delete a book
    #[instrument(skip(pool))]
    pub async fn delete(pool: &PgPool, id: String) -> AppResult<u64> {
        let result = sqlx::query!(
            r#"
                DELETE FROM book
                WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected())
    }

    /// Update a book
    #[instrument(skip(pool))]
    pub async fn update(pool: &PgPool, id: String, book: &BookCreation) -> AppResult<()> {
        sqlx::query!(
            r#"
                UPDATE book
                SET title = $1, author = $2, updated_at = $3
                WHERE id = $4
            "#,
            book.title,
            book.author,
            Some(Utc::now()),
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Get amount of existing books
    #[instrument(skip(pool))]
    async fn get_total(pool: &PgPool) -> Result<i64, sqlx::Error> {
        let query = r#"
            SELECT COUNT(id) AS n
            FROM book
        "#;

        Ok(sqlx::query(query).fetch_one(pool).await?.get("n"))
    }
}
