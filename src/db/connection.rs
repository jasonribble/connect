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
    use crate::{
        db::{fake_db::test_helpers, Connection, ContactRepo, MetadataRepo},
        models::Contact,
    };

    #[tokio::test]
    async fn test_create_contact_get_metadata() {
        let pool = test_helpers::setup_in_memory_db().await;

        let data_repo = Connection::new(pool);

        let example_contact =
            Contact::new("Lewis", "Carroll", "lewis@wonderland.com", "777-777-7777").unwrap();

        let result_contact_id = data_repo.create_contact(example_contact).await;
        let contact_id = result_contact_id.unwrap();

        let result_expected_metadata = data_repo.get_metadata_by_id(contact_id).await;

        let expected_metadata = result_expected_metadata.unwrap();

        assert_eq!(contact_id, expected_metadata.contact_id);
    }

    #[tokio::test]
    async fn test_delete_contact_deletes_metadata() {
        let pool = test_helpers::setup_in_memory_db().await;

        let data_repo = Connection::new(pool);

        let example_contact =
            Contact::new("Lewis", "Carroll", "lewis@wonderland.com", "777-777-7777").unwrap();

        let result_contact_id = data_repo.create_contact(example_contact.clone()).await;

        let contact_id = result_contact_id.unwrap();

        let contact_from_database = data_repo.get_contact_by_id(contact_id).await;
        let contact_from_database = contact_from_database.unwrap();

        assert_eq!(
            contact_from_database.contact.first_name,
            example_contact.first_name
        );

        let deleted_contact_id = data_repo.delete_contact_by_id(contact_id).await.unwrap();

        let failed_contact_metadata = data_repo.get_metadata_by_id(deleted_contact_id).await;

        assert!(failed_contact_metadata.is_err());
    }
}
