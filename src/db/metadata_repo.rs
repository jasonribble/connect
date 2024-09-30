use crate::models::{self, Metadata};
use async_trait::async_trait;
use chrono::SecondsFormat;

use super::Connection;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait MetadataRepo {
    async fn create_metadata(&self, contact_id: i64) -> anyhow::Result<Metadata>;
    async fn get_metadata_by_id(&self, contact_id: i64) -> anyhow::Result<models::Metadata>;
}

#[async_trait]
impl MetadataRepo for Connection {
    async fn create_metadata(&self, contact_id: i64) -> anyhow::Result<Metadata> {
        let query = "INSERT INTO contacts_metadata 
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

        let metadata = Metadata::new(contact_id);

        sqlx::query(query)
            .bind(metadata.contact_id)
            .bind(metadata.starred)
            .bind(metadata.is_archived)
            .bind(metadata.frequency.clone())
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

        // Fetch the inserted metadata
        self.get_metadata_by_id(contact_id).await
    }
    async fn get_metadata_by_id(&self, contact_id: i64) -> anyhow::Result<models::Metadata> {
        let query_get_by_id = "SELECT * FROM contacts_metadata WHERE contact_id=$1";

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
            "CREATE TABLE IF NOT EXISTS contacts_metadata (
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
        .expect("Failed to create contacts_metadata table");

        pool
    }
    #[tokio::test]
    async fn test_create_metadata_sqlite() {
        let pool = setup_test_db().await;
        let repo = Connection::new(pool);

        let contact_id = 1;

        let result = repo.create_metadata(contact_id).await.unwrap();

        assert_eq!(result.contact_id, contact_id);
    }

    #[tokio::test]
    async fn test_create_metadata() {
        let mut mock_metadata_repo = MockMetadataRepo::new();

        let test_metadata = models::Metadata::new(1);
        let returning_metadata = models::Metadata::new(1);

        mock_metadata_repo
            .expect_create_metadata()
            .times(1)
            .returning(move |_| Ok(returning_metadata.clone()));

        let result = mock_metadata_repo
            .create_metadata(test_metadata.contact_id)
            .await;

        let result = result.unwrap();

        assert_eq!(result.contact_id, 1);
    }

    #[tokio::test]
    async fn test_get_metadata() {
        let mut mock_metadata_repo = MockMetadataRepo::new();

        let test_metadata = models::Metadata::new(1);

        // Clone test_metadata before using it in the closure
        let test_metadata_clone = test_metadata.clone();

        mock_metadata_repo
            .expect_get_metadata_by_id()
            .times(1)
            .with(eq(1))
            .returning(move |_| Ok(test_metadata_clone.clone()));

        let result = mock_metadata_repo.get_metadata_by_id(1).await;

        assert!(result.is_ok());

        let expected_metadata = result.unwrap();

        assert_eq!(expected_metadata, test_metadata);
    }
}
