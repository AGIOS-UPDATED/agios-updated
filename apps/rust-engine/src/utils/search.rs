use serde::{de::DeserializeOwned, Serialize};
use std::error::Error as StdError; // Renamed to avoid confusion
use sqlx::{Pool, Postgres, FromRow};

pub trait SearchDocument: Serialize + DeserializeOwned {
    fn get_id(&self) -> &str;
}

pub struct SearchClient<T> {
    search: Search<T>,
}

impl<T> SearchClient<T>
where
    T: Send + Sync + Unpin + for<'r> FromRow<'r, sqlx::postgres::PgRow>,
{
    /// Create a new `SearchClient` with a given connection pool.
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self {
            search: Search::new(pool),
        }
    }

    /// Search method that runs the query against the underlying `Search`.
    ///
    /// Uses the `?` operator to propagate any `sqlx::Error`, automatically converting
    /// it into `Box<dyn Error>` due to the `From<sqlx::Error> for Box<dyn StdError>` impl.
    pub async fn search(&self, query: &str) -> Result<Vec<T>, Box<dyn StdError>> {
        let results = self.search.execute(query).await?;
        Ok(results)
    }

    /// Stub for an update method. Not yet implemented.
    pub async fn update_document(
        &self,
        _id: &str,
        _update_fn: impl FnOnce(T) -> Result<T, Box<dyn StdError>>
    ) -> Result<(), Box<dyn StdError>> {
        // TODO: Implement update_document
        todo!()
    }
}

pub struct Search<T> {
    pool: Pool<Postgres>,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Search<T>
where
    T: Send + Sync + Unpin + for<'r> FromRow<'r, sqlx::postgres::PgRow>,
{
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self {
            pool,
            _marker: std::marker::PhantomData,
        }
    }

    /// Execute the SQL query as typed query returning a `Vec<T>`.
    ///
    /// `sqlx::query_as` returns `Result<Vec<T>, sqlx::Error>`.
    pub async fn execute(&self, query: &str) -> Result<Vec<T>, sqlx::Error> {
        sqlx::query_as::<_, T>(query)
            .fetch_all(&self.pool)
            .await
    }
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}

impl<T: Serialize> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, total: i64, page: i64, per_page: i64) -> Self {
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        Self {
            data,
            total,
            page,
            per_page,
            total_pages,
        }
    }
}
