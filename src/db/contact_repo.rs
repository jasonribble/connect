use std::sync::Arc;

use crate::models;
use async_trait::async_trait;
use sqlx::SqlitePool;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait ContactRepo {
    async fn create(&self, contact: models::Contact) -> anyhow::Result<i64>;
    async fn get_all(&self) -> anyhow::Result<Vec<models::IndexedContact>>;
    async fn update(&self, update: models::ContactBuilder) -> anyhow::Result<()>;
    async fn get_by_id(&self, id: i64) -> anyhow::Result<models::IndexedContact>;
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
impl ContactRepo for Connection {
    async fn create(&self, contact: models::Contact) -> anyhow::Result<i64> {
        let query = "INSERT INTO contacts
        (first_name, last_name, display_name, email, phone_number)
        VALUES (?, ?, ?, ?, ?)";
        let result = sqlx::query(query)
            .bind(&contact.first_name)
            .bind(&contact.last_name)
            .bind(&contact.display_name)
            .bind(&contact.email)
            .bind(&contact.phone_number)
            .execute(&*self.sqlite_pool)
            .await?;

        Ok(result.last_insert_rowid())
    }

    async fn get_all(&self) -> anyhow::Result<Vec<models::IndexedContact>> {
        let get_contacts_query =
            "SELECT id, first_name, last_name, display_name, email, phone_number
             FROM contacts
             ORDER BY id";

        let contacts_with_id: Vec<models::IndexedContact> =
            sqlx::query_as::<_, models::IndexedContact>(get_contacts_query)
                .fetch_all(&*self.sqlite_pool)
                .await?;

        Ok(contacts_with_id)
    }

    async fn update(&self, contact: models::ContactBuilder) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            UPDATE contacts
            SET
                first_name = COALESCE($1, first_name),
                last_name = COALESCE($2, last_name),
                display_name = COALESCE($3, display_name),
                email = COALESCE($4, email),
                phone_number = COALESCE($5, phone_number)
            WHERE id = $6
            "#,
            contact.update.first_name,
            contact.update.last_name,
            contact.update.display_name,
            contact.update.email,
            contact.update.phone_number,
            contact.id
        )
        .execute(&*self.sqlite_pool)
        .await?;

        println!("Contact updated");

        Ok(())
    }

    async fn get_by_id(&self, id: i64) -> anyhow::Result<models::IndexedContact> {
        let query_get_by_id = "SELECT * FROM contacts WHERE id=$1";

        let contact: models::IndexedContact =
            sqlx::query_as::<_, models::IndexedContact>(query_get_by_id)
                .bind(id)
                .fetch_one(&*self.sqlite_pool)
                .await?;

        Ok(contact)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_save_contact() {
        let mut mock_contact_repo = MockContactRepo::new();

        let test_contact =
            models::Contact::new("John", "Smith", "johndoe@example.com", "123-456-7890").unwrap();

        mock_contact_repo
            .expect_create()
            .times(1)
            .with(eq(test_contact.clone()))
            .returning(|_| Ok(1));

        let result = mock_contact_repo.create(test_contact).await;

        let result = result.unwrap();

        assert_eq!(result, 1);
    }

    #[tokio::test]
    async fn test_get_all_contacts() {
        let mut mock_contact_repo = MockContactRepo::new();

        let contacts = vec![models::IndexedContact {
            id: 1,
            contact: models::Contact::new("John", "Doe", "johndoe@example.com", "1234567890")
                .unwrap(),
        }];

        mock_contact_repo
            .expect_get_all()
            .times(1)
            .return_once(move || Ok(contacts));

        let result = mock_contact_repo.get_all().await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_contact() {
        let mut mock_contact_repo = MockContactRepo::new();

        mock_contact_repo
            .expect_update()
            .times(1)
            .return_once(|_| Ok(()));

        let edits = models::ContactBuilder::new(
            1,
            None,
            None,
            Some("some@email.com".to_string()),
            None,
            None,
        )
        .unwrap();

        let result = mock_contact_repo.update(edits).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_contact_by_id() {
        let mut mock_contact_repo = MockContactRepo::new();

        let contact = models::IndexedContact {
            id: 1,
            contact: models::Contact::new("John", "Doe", "johndoe@example.com", "1234567890")
                .unwrap(),
        };

        mock_contact_repo
            .expect_get_by_id()
            .times(1)
            .with(eq(contact.id))
            .return_once(|_| Ok(contact));

        let result = mock_contact_repo.get_by_id(1).await;

        assert!(result.is_ok());

        let actual_contact = result.unwrap();

        assert_eq!(actual_contact.id, 1);
    }
}
