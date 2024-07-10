use std::sync::Arc;

use crate::models;
use async_trait::async_trait;
use sqlx::postgres::PgPool;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait ContactRepo {
    async fn save_contact(&self, contact: models::Contact) -> anyhow::Result<i64>;
    async fn get_all(&self) -> anyhow::Result<Vec<models::IndexedContact>>;
    async fn update_contact(&self) -> anyhow::Result<()>;
}

pub struct PostgresContactRepo {
    pg_pool: Arc<PgPool>,
}

impl PostgresContactRepo {
    pub fn new(pg_pool: PgPool) -> Self {
        Self {
            pg_pool: Arc::new(pg_pool),
        }
    }
}

#[async_trait]
impl ContactRepo for PostgresContactRepo {
    async fn save_contact(&self, contact: models::Contact) -> anyhow::Result<i64> {
        let query = "INSERT INTO contacts
        (first_name, last_name, display_name, email, phone_number)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id";

        let (id,) = sqlx::query_as::<_, (i64,)>(query)
            .bind(&contact.first_name)
            .bind(&contact.last_name)
            .bind(&contact.display_name)
            .bind(&contact.email)
            .bind(&contact.phone_number)
            .fetch_one(&*self.pg_pool)
            .await?;

        Ok(id)
    }

    async fn get_all(&self) -> anyhow::Result<Vec<models::IndexedContact>> {
        let get_contacts_query =
            "SELECT id, first_name, last_name, display_name, email, phone_number
             FROM contacts
             ORDER BY id";

        let contacts_with_id: Vec<models::IndexedContact> =
            sqlx::query_as::<_, models::IndexedContact>(get_contacts_query)
                .fetch_all(&*self.pg_pool)
                .await?;

        Ok(contacts_with_id)
    }

    async fn update_contact(&self) -> anyhow::Result<()> {
        Ok(())
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
            .expect_save_contact()
            .times(1)
            .with(eq(test_contact.clone()))
            .returning(|_| Ok(1));

        let result = mock_contact_repo.save_contact(test_contact).await;

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
            .expect_update_contact()
            .times(1)
            .return_once(move || Ok(()));

        let result = mock_contact_repo.update_contact().await;

        assert!(result.is_ok());
    }
}
