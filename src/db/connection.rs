use sqlx::SqlitePool;
use std::sync::Arc;
pub struct Connection {
    pub sqlite_pool: Arc<SqlitePool>,
}

impl Connection {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            sqlite_pool: Arc::new(pool),
        }
    }
}

#[cfg(test)]
mod tests {
    use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

    use crate::{
        db::{Connection, ContactRepo, MetadataRepo},
        models::Contact,
    };

    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .expect("Failed to create in-memory SQLite database");

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS contacts
          (
          id INTEGER PRIMARY KEY AUTOINCREMENT,
          first_name TEXT NOT NULL,
          last_name TEXT NOT NULL,
          display_name TEXT NOT NULL,
          email TEXT NOT NULL,
          phone_number TEXT NOT NULL
          );",
        )
        .execute(&pool)
        .await
        .expect("Failed to create contact table");

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
    async fn test_create_contact_get_metadata() {
        let pool = setup_test_db().await;

        let data_repo = Connection::new(pool);

        let example_contact =
            Contact::new("Lewis", "Carroll", "lewis@wonderland.com", "777-777-7777").unwrap();

        let result_contact_id = data_repo.create_contact(example_contact).await;
        let contact_id = result_contact_id.unwrap();

        let result_expected_metadata = data_repo.get_metadata_by_id(contact_id).await;

        let expected_metadata = result_expected_metadata.unwrap();

        assert_eq!(contact_id, expected_metadata.contact_id);
    }
}
