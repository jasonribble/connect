use crate::{
    models,
    utils::{self, default_date},
};
use async_trait::async_trait;
use sqlx::SqlitePool;

use super::connection::Repo;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait ContactRepo {
    async fn save_contact(&self, contact: models::Contact) -> anyhow::Result<i64>;
    async fn save_optional_contact(&self, contact: models::OptionalContact) -> anyhow::Result<i64>;
    async fn import_contacts_by_csv(&self, filename: &str) -> anyhow::Result<i64>;
    async fn get_all_contacts(&self) -> anyhow::Result<Vec<models::IndexedContact>>;
    async fn update_contact(&self, update: models::ContactBuilder) -> anyhow::Result<()>;
    async fn get_contact_by_id(&self, id: i64) -> anyhow::Result<models::IndexedContact>;
    async fn delete_contact_by_id(&self, id: i64) -> anyhow::Result<i64>;
}

#[async_trait]
impl ContactRepo for Repo<SqlitePool> {
    async fn save_contact(&self, contact: models::Contact) -> anyhow::Result<i64> {
        let query = "INSERT INTO contacts
        (first_name, last_name, display_name, email, phone_number, birthday, starred, is_archived, created_at, updated_at, last_seen_at, frequency, last_reminder_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";
        let result = sqlx::query(query)
            .bind(&contact.first_name)
            .bind(&contact.last_name)
            .bind(&contact.display_name)
            .bind(&contact.email)
            .bind(&contact.phone_number)
            .bind(contact.birthday)
            .bind(contact.starred)
            .bind(contact.is_archived)
            .bind(contact.created_at)
            .bind(contact.updated_at)
            .bind(contact.last_seen_at)
            .bind(&contact.frequency)
            .bind(contact.last_reminder_at)
            .execute(&*self.database)
            .await?;

        let contact_id = result.last_insert_rowid();

        Ok(contact_id)
    }

    async fn get_all_contacts(&self) -> anyhow::Result<Vec<models::IndexedContact>> {
        let get_contacts_query = "SELECT *
             FROM contacts
             ORDER BY id";

        let contacts_with_id: Vec<models::IndexedContact> =
            sqlx::query_as::<_, models::IndexedContact>(get_contacts_query)
                .fetch_all(&*self.database)
                .await?;

        Ok(contacts_with_id)
    }

    async fn update_contact(&self, contact: models::ContactBuilder) -> anyhow::Result<()> {
        use chrono::Utc;
        let now = Utc::now();
        sqlx::query!(
            r#"
            UPDATE contacts
            SET
                first_name = COALESCE($1, first_name),
                last_name = COALESCE($2, last_name),
                display_name = COALESCE($3, display_name),
                email = COALESCE($4, email),
                phone_number = COALESCE($5, phone_number),
                birthday = COALESCE($6, birthday),
                starred = COALESCE($7, starred),
                is_archived = COALESCE($8, is_archived),
                updated_at = $9,
                last_seen_at = COALESCE($10, last_seen_at),
                frequency = COALESCE($11, frequency),
                last_reminder_at = COALESCE($12, last_reminder_at)
            WHERE id = $13
            "#,
            contact.optional_contact.first_name,
            contact.optional_contact.last_name,
            contact.optional_contact.display_name,
            contact.optional_contact.email,
            contact.optional_contact.phone_number,
            contact.optional_contact.birthday,
            contact.optional_contact.starred,
            contact.optional_contact.is_archived,
            now,
            contact.optional_contact.last_seen_at,
            contact.optional_contact.frequency,
            contact.optional_contact.last_reminder_at,
            contact.id
        )
        .execute(&*self.database)
        .await?;

        Ok(())
    }

    async fn get_contact_by_id(&self, id: i64) -> anyhow::Result<models::IndexedContact> {
        let query_get_by_id = "SELECT * FROM contacts WHERE id=$1";

        let contact: models::IndexedContact =
            sqlx::query_as::<_, models::IndexedContact>(query_get_by_id)
                .bind(id)
                .fetch_one(&*self.database)
                .await?;

        Ok(contact)
    }

    async fn delete_contact_by_id(&self, id: i64) -> anyhow::Result<i64> {
        let query_delete_by_id = "DELETE FROM contacts WHERE id=$1";

        let result = sqlx::query(query_delete_by_id)
            .bind(id)
            .execute(&*self.database)
            .await?;

        if result.rows_affected() == 0 {
            anyhow::bail!("That Contact ID does not exist")
        }

        Ok(id)
    }

    async fn save_optional_contact(&self, contact: models::OptionalContact) -> anyhow::Result<i64> {
        use chrono::Utc;

        let mut display_name = contact.display_name.clone();

        if display_name.is_none() {
            let firstname = contact.first_name.clone();
            let lastname = contact.last_name.clone();
            display_name = Some(format!(
                "{} {}",
                firstname.unwrap_or_default(),
                lastname.unwrap_or_default()
            ));
        }

        let query =
            "INSERT INTO contacts (first_name, last_name, display_name, phone_number, email, birthday, starred, is_archived, created_at, updated_at, last_seen_at, frequency, last_reminder_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";

        let default_date = default_date();
        let birthday = contact.birthday.unwrap_or(default_date);

        let now = Utc::now();

        let result = sqlx::query(query)
            .bind(&contact.first_name)
            .bind(&contact.last_name)
            .bind(display_name)
            .bind(&contact.phone_number)
            .bind(&contact.email)
            .bind(birthday)
            .bind(contact.starred.unwrap_or(false))
            .bind(contact.is_archived.unwrap_or(false))
            .bind(now)
            .bind(now)
            .bind(contact.last_seen_at)
            .bind(&contact.frequency)
            .bind(contact.last_reminder_at)
            .execute(&*self.database)
            .await?;

        let contact_id = result.last_insert_rowid();

        Ok(contact_id)
    }

    #[allow(clippy::arithmetic_side_effects)]
    async fn import_contacts_by_csv(&self, filename: &str) -> anyhow::Result<i64> {
        let contacts = utils::process_csv_to_contacts(filename)?;

        let mut number_of_contacts_added = 0;
        for contact in &contacts {
            self.save_optional_contact(contact.clone()).await?;
            number_of_contacts_added += 1;
        }

        Ok(number_of_contacts_added)
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
#[allow(clippy::panic_in_result_fn)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::test_helpers::setup_in_memory_db;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_save_contact() {
        let mut mock_contact_repo = MockContactRepo::new();

        let test_contact = models::Contact::builder()
            .first_name("John")
            .last_name("Smith")
            .email("johndoe@example.com")
            .phone_number("123-456-7890")
            .birthday("1970-1-1")
            .build()
            .expect("Test contact");

        mock_contact_repo
            .expect_save_contact()
            .times(1)
            .with(eq(test_contact.clone()))
            .returning(|_| Ok(1));

        let result = mock_contact_repo
            .save_contact(test_contact)
            .await
            .expect("Saved contact");

        assert_eq!(result, 1);
    }

    #[tokio::test]
    async fn test_get_all_contacts() {
        let mut mock_contact_repo = MockContactRepo::new();

        let contacts = vec![models::IndexedContact {
            id: 1,
            contact: models::Contact::builder()
                .first_name("John")
                .last_name("Doe")
                .email("johndoe@example.com")
                .phone_number("1234567890")
                .birthday("1970-01-01")
                .build()
                .expect("Test contact"),
        }];

        mock_contact_repo
            .expect_get_all_contacts()
            .times(1)
            .return_once(move || Ok(contacts));

        let result = mock_contact_repo.get_all_contacts().await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_contact() {
        let mut mock_contact_repo = MockContactRepo::new();

        mock_contact_repo
            .expect_update_contact()
            .times(1)
            .return_once(|_| Ok(()));

        let edits = models::ContactBuilder::new(
            1,
            None,
            None,
            Some("some@email.com".to_string()),
            None,
            None,
            None,
        )
        .expect("Contact with only an email");

        let result = mock_contact_repo.update_contact(edits).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_contact_by_id() {
        let mut mock_contact_repo = MockContactRepo::new();

        let contact = models::IndexedContact {
            id: 1,
            contact: models::Contact::builder()
                .first_name("John")
                .last_name("Doe")
                .email("johndoe@example.com")
                .phone_number("1234567890")
                .birthday("1970-01-01")
                .build()
                .expect("Default contact"),
        };

        mock_contact_repo
            .expect_get_contact_by_id()
            .times(1)
            .with(eq(contact.id))
            .return_once(|_| Ok(contact));

        let result = mock_contact_repo.get_contact_by_id(1).await;

        assert!(result.is_ok());

        let actual_contact = result.expect("Actual Contact");

        assert_eq!(actual_contact.id, 1);
    }

    #[tokio::test]
    async fn should_save_option_contact_in_database() -> anyhow::Result<()> {
        let pool = setup_in_memory_db().await;

        let data_repo = Repo::new(pool);

        let test_contact = models::OptionalContact {
            first_name: Some("Jason".to_string()),
            ..models::OptionalContact::default()
        };

        let result = data_repo.save_optional_contact(test_contact).await?;

        assert_eq!(result, 1);

        Ok(())
    }

    #[tokio::test]
    async fn should_be_able_to_retrieve_full_contact_when_saved_full_contact() -> anyhow::Result<()>
    {
        let pool = setup_in_memory_db().await;

        let data_repo = Repo::new(pool);

        let test_contact = models::OptionalContact {
            first_name: Some("Ada".to_string()),
            last_name: Some("Lovelace".to_string()),
            display_name: Some("Addy".to_string()),
            email: Some("ada@lovelace.rs".to_string()),
            phone_number: Some("1233211233".to_string()),
            birthday: Some(chrono::NaiveDate::default()),
            starred: None,
            is_archived: None,
            last_seen_at: None,
            frequency: None,
            last_reminder_at: None,
        };

        let contact_id = data_repo
            .save_optional_contact(test_contact.clone())
            .await?;

        let saved_contact = data_repo.get_contact_by_id(contact_id).await?;

        let saved_contact = saved_contact.contact;

        assert_eq!(
            saved_contact.first_name,
            test_contact.first_name.expect("Contact first name")
        );
        assert_eq!(
            saved_contact.last_name,
            test_contact.last_name.expect("Contact last name")
        );
        assert_eq!(
            saved_contact.phone_number,
            test_contact.phone_number.expect("Contact phone number")
        );

        assert_eq!(
            saved_contact.email,
            test_contact.email.expect("Contact emails")
        );

        assert_eq!(
            saved_contact.display_name,
            test_contact.display_name.expect("Display name")
        );

        Ok(())
    }

    #[tokio::test]
    async fn should_default_to_first_and_last_name_for_display_name() -> anyhow::Result<()> {
        let pool = setup_in_memory_db().await;

        let data_repo = Repo::new(pool);

        let test_contact = models::OptionalContact {
            first_name: Some("Ada".to_string()),
            last_name: Some("Lovelace".to_string()),
            birthday: Some(chrono::NaiveDate::default()),
            ..models::OptionalContact::default()
        };

        let contact_id = data_repo
            .save_optional_contact(test_contact.clone())
            .await?;

        let saved_contact = data_repo.get_contact_by_id(contact_id).await?;

        let saved_contact = saved_contact.contact;

        assert_eq!(saved_contact.display_name, "Ada Lovelace".to_string());

        let test_contact = models::OptionalContact {
            first_name: Some("Jason".to_string()),
            last_name: Some("Ribble".to_string()),
            ..models::OptionalContact::default()
        };

        let contact_id = data_repo
            .save_optional_contact(test_contact.clone())
            .await?;

        let saved_contact = data_repo.get_contact_by_id(contact_id).await?;

        let saved_contact = saved_contact.contact;

        assert_eq!(saved_contact.display_name, "Jason Ribble".to_string());

        Ok(())
    }

    #[tokio::test]
    async fn should_save_two_option_contact_in_database() -> anyhow::Result<()> {
        let pool = setup_in_memory_db().await;

        let data_repo = Repo::new(pool);

        let test_contact = models::OptionalContact {
            first_name: Some("Jason".to_string()),
            ..models::OptionalContact::default()
        };

        let contact_id = data_repo.save_optional_contact(test_contact).await?;

        assert_eq!(contact_id, 1);

        let another_contact = models::OptionalContact {
            first_name: Some("Alice".to_string()),
            ..models::OptionalContact::default()
        };

        let contact_id = data_repo.save_optional_contact(another_contact).await?;

        assert_eq!(contact_id, 2);

        Ok(())
    }

    #[tokio::test]
    async fn should_store_one_contact_when_given_alice_csv() -> anyhow::Result<()> {
        let pool = setup_in_memory_db().await;

        let data_repo = Repo::new(pool);

        let example_csv = "tests/fixtures/alice.csv";

        let number_of_imported_contacts = Ok(data_repo.import_contacts_by_csv(example_csv).await?);

        let number_of_contacts = i64::try_from(data_repo.get_all_contacts().await?.len());

        assert_eq!(number_of_contacts, number_of_imported_contacts);

        Ok(())
    }

    #[tokio::test]
    async fn should_store_three_contacts_when_given_example() -> anyhow::Result<()> {
        let pool = setup_in_memory_db().await;

        let data_repo = Repo::new(pool);

        let example_csv = "tests/fixtures/example.csv";

        let number_of_imported_contacts = Ok(data_repo.import_contacts_by_csv(example_csv).await?);

        let number_of_contacts = i64::try_from(data_repo.get_all_contacts().await?.len());

        assert_eq!(number_of_contacts, number_of_imported_contacts);

        Ok(())
    }

    #[tokio::test]
    async fn should_create_metadata_when_importing_csv() -> anyhow::Result<()> {
        let pool = setup_in_memory_db().await;

        let data_repo = Repo::new(pool);

        let example_csv = "tests/fixtures/example.csv";

        let number_of_imported_contacts = data_repo.import_contacts_by_csv(example_csv).await?;

        let contacts = data_repo.get_all_contacts().await?;
        let last_contact = contacts.last().expect("Contact has a last item");
        assert_eq!(number_of_imported_contacts, last_contact.id);

        Ok(())
    }

    #[tokio::test]
    async fn should_save_csv_with_one_row_and_birthday() -> anyhow::Result<()> {
        let pool = setup_in_memory_db().await;

        let data_repo = Repo::new(pool);

        let example_csv = "tests/fixtures/birthday.csv";

        data_repo.import_contacts_by_csv(example_csv).await?;

        let contacts = data_repo.get_all_contacts().await?;

        let aldous_huxley_birthday = contacts.first().unwrap().contact.birthday;

        assert_eq!(
            aldous_huxley_birthday,
            chrono::NaiveDate::from_ymd_opt(1894, 7, 26).expect("Aldous's Birthday")
        );

        Ok(())
    }

    #[tokio::test]
    async fn should_save_csv_with_threes_row_and_birthday() -> anyhow::Result<()> {
        let pool = setup_in_memory_db().await;

        let data_repo = Repo::new(pool);

        let example_csv = "tests/fixtures/three_birthdays.csv";

        data_repo.import_contacts_by_csv(example_csv).await?;

        let contacts = data_repo.get_all_contacts().await?;

        let aldous_huxley_birthday = contacts.first().unwrap().contact.birthday;

        assert_eq!(
            aldous_huxley_birthday,
            chrono::NaiveDate::from_ymd_opt(1894, 7, 26).expect("Valid date")
        );

        let cs_lewis_birthday = contacts.get(2).unwrap().contact.birthday;

        assert_eq!(
            cs_lewis_birthday,
            chrono::NaiveDate::from_ymd_opt(1898, 11, 29).expect("Valid date")
        );

        Ok(())
    }

    #[tokio::test]
    async fn should_return_error_when_deleting_nonexistent_id() {
        let pool = setup_in_memory_db().await;
        let data_repo = Repo::new(pool);

        let nonexistent_id = 999;
        let result = data_repo.delete_contact_by_id(nonexistent_id).await;

        assert!(result.is_err());
    }
}
