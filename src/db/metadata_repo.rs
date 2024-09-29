use std::sync::Arc;

use crate::models;
use async_trait::async_trait;
use chrono::SecondsFormat;
use sqlx::SqlitePool;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait MetadataRepo {
    async fn create(&self, metadata: models::Metadata) -> anyhow::Result<i64>;
    async fn get_by_id(&self, contact_id: i64) -> anyhow::Result<models::Metadata>;
}

pub struct Connection {
    sqlite_pool: Arc<SqlitePool>,
}

impl Connection {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            sqlite_pool: Arc::new(pool),
        }
    }
}

#[async_trait]
impl MetadataRepo for Connection {
    async fn create(&self, metadata: models::Metadata) -> anyhow::Result<i64> {
        let query = "INSERT INTO contact_metadata 
    (contact_id, 
     starred, 
     is_archived, 
     frequency,
     created_at,
     updated_at,
     last_seen_at, 
     next_reminder_at, 
     last_reminder_at) 
     VALUES (?,?,?,?,?,?,?,?,?)";

        let result = sqlx::query(query)
            .bind(metadata.contact_id)
            .bind(metadata.starred)
            .bind(metadata.is_archived)
            .bind(metadata.frequency)
            .bind(
                metadata
                    .created_at
                    .to_rfc3339_opts(SecondsFormat::Millis, true),
            )
            .bind(
                metadata
                    .updated_at
                    .to_rfc3339_opts(SecondsFormat::Millis, true),
            )
            .bind(
                metadata
                    .last_seen_at
                    .map(|dt| dt.to_rfc3339_opts(SecondsFormat::Millis, true)),
            )
            .bind(
                metadata
                    .next_reminder_at
                    .map(|dt| dt.to_rfc3339_opts(SecondsFormat::Millis, true)),
            )
            .bind(
                metadata
                    .last_reminder_at
                    .map(|dt| dt.to_rfc3339_opts(SecondsFormat::Millis, true)),
            )
            .execute(&*self.sqlite_pool)
            .await?;

        Ok(result.last_insert_rowid())
    }
    async fn get_by_id(&self, contact_id: i64) -> anyhow::Result<models::Metadata> {
        let query_get_by_id = "SELECT * FROM metadata WHERE contact_id=$1";

        let metadata: models::Metadata = sqlx::query_as::<_, models::Metadata>(query_get_by_id)
            .bind(contact_id)
            .fetch_one(&*self.sqlite_pool)
            .await?;

        Ok(metadata)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models;
    use mockall::predicate::*;
    use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .expect("Failed to create in-memory SQLite database");

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS contact_metadata (
                contact_id INTEGER NOT NULL,
                starred BOOLEAN NOT NULL,
                is_archived BOOLEAN NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                last_seen_at TEXT,
                next_reminder_at TEXT,
                frequency INTEGER,
                last_reminder_at TEXT
            )",
        )
        .execute(&pool)
        .await
        .expect("Failed to create contact_metadata table");

        pool
    }
    #[tokio::test]
    async fn test_create_metadata_sqlite() {
        let pool = setup_test_db().await;
        let repo = Connection::new(pool);

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

    #[tokio::test]
    async fn test_get_metadata() {
        let mut mock_metadata_repo = MockMetadataRepo::new();

        let test_metadata = models::Metadata {
            contact_id: 1,
            ..models::Metadata::default()
        };

        // Clone test_metadata before using it in the closure
        let test_metadata_clone = test_metadata.clone();

        mock_metadata_repo
            .expect_get_by_id()
            .times(1)
            .with(eq(1))
            .returning(move |_| Ok(test_metadata_clone.clone()));

        let result = mock_metadata_repo.get_by_id(1).await;

        assert!(result.is_ok());

        let expected_metadata = result.unwrap();

        assert_eq!(expected_metadata, test_metadata);
    }
}
