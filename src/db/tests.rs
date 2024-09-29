use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

use crate::{
    db::{ContactConnection, ContactRepo, MetadataConnection, MetadataRepo},
    models::{Contact, Metadata},
};

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

    pool
}

#[tokio::test]
async fn test_create_contact_get_metadata() {
    let pool = setup_test_db().await;

    let contact_repo = ContactConnection::new(pool.clone());
    let metadata_repo = MetadataConnection::new(pool.clone());

    let example_contact =
        Contact::new("Lewis", "Carroll", "lewis@wonderland.com", "777-777-7777").unwrap();

    let result_contact_id = contact_repo.create(example_contact).await;
    let contact_id = result_contact_id.unwrap();

    assert_eq!(contact_id, 1);

    let default_metadata = Metadata::default();

    let result_expected_metadata = metadata_repo.get_by_id(contact_id).await;
    let expected_metadata = result_expected_metadata.unwrap();

    assert_eq!(default_metadata, expected_metadata);
}
