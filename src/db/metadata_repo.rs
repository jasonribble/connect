use std::sync::Arc;

use crate::models;
use async_trait::async_trait;
use sqlx::SqlitePool;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait MetadataRepo {
    async fn create(&self, metadata: models::Metadata) -> anyhow::Result<i64>;
}

pub struct SqliteMetadataRepo {
    sqlite_pool: Arc<SqlitePool>,
}

impl SqliteMetadataRepo {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            sqlite_pool: Arc::new(pool),
        }
    }
}

#[async_trait]
impl MetadataRepo for SqliteMetadataRepo {
   async fn create(&self, metadata: models::Metadata) -> anyhow::Result<i64> {
      todo!();
   }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use crate::models;
    use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

     
    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .expect("Failed to create in-memory SQLite database");
        
        sqlx::query("CREATE TABLE metadata (id INTEGER PRIMARY KEY, name TEXT)")
            .execute(&pool)
            .await
            .expect("Failed to create metadata table");
        
        pool
    }

    #[tokio::test]
    async fn test_create_metadata_sqlite() {
    let pool = setup_test_db().await;
    let repo = SqliteMetadataRepo::new(pool);

    let test_metadata = models::Metadata::default();

    let result = repo.create(test_metadata.clone()).await.unwrap();
    assert!(result > 0);
}
    

    #[tokio::test]
    async fn test_create_metadata() {
        let mut mock_metadata_repo = MockMetadataRepo::new();

        let test_metadata = models::Metadata::default();


        mock_metadata_repo
            .expect_create()
            .times(1)
            .with(eq(test_metadata.clone()))
            .returning(|_| Ok(1));

        let result = mock_metadata_repo.create(test_metadata).await;

        let result = result.unwrap();

        assert_eq!(result, 1);
    }
}


