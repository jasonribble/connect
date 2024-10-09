use crate::utils;
use crate::utils::AppError;

#[derive(Debug, PartialEq, Eq, Clone, sqlx::FromRow)]
pub struct Contact {
    pub first_name: String,
    pub last_name: String,
    pub display_name: String,
    pub email: String,
    pub phone_number: String,
}

#[derive(Debug, PartialEq, Eq, Clone, sqlx::FromRow)]
pub struct Indexed {
    pub id: i64,
    #[sqlx(flatten)]
    pub contact: Contact,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Update {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
}

impl Update {
    pub const fn is_empty(&self) -> bool {
        self.first_name.is_none()
            && self.last_name.is_none()
            && self.display_name.is_none()
            && self.email.is_none()
            && self.phone_number.is_none()
    }
}

#[derive(Debug)]
pub struct Construct {
    pub id: i64,
    pub update: Update,
}
impl Construct {
    pub fn new(
        id: i64,
        first_name: Option<String>,
        last_name: Option<String>,
        email: Option<String>,
        phone_number: Option<String>,
        display_name: Option<String>,
    ) -> Result<Self, AppError> {
        let maybe_email = email.as_deref().unwrap_or("");

        if utils::is_not_valid_email(maybe_email) && Option::is_some(&email) {
            return Err(AppError::InvalidEmail(email.clone().unwrap_or_default()));
        }

        let maybe_phone = phone_number.as_deref().unwrap_or("");

        if utils::is_not_valid_phone_number(maybe_phone) && Option::is_some(&phone_number) {
            return Err(AppError::InvalidPhoneNumber(
                phone_number.clone().unwrap_or_default(),
            ));
        }

        let update = Update {
            first_name,
            last_name,
            display_name,
            email,
            phone_number,
        };

        if update.is_empty() {
            return Err(AppError::EmptyUpdate);
        }

        Ok(Self { id, update })
    }

    #[allow(dead_code)]
    const fn is_empty(&self) -> bool {
        self.update.first_name.is_none()
            && self.update.last_name.is_none()
            && self.update.display_name.is_none()
            && self.update.email.is_none()
            && self.update.phone_number.is_none()
    }
}

impl Contact {
    pub fn new(
        first_name: &str,
        last_name: &str,
        email: &str,
        phone_number: &str,
    ) -> Result<Self, AppError> {
        let display_name = format!("{first_name} {last_name}");

        if utils::is_not_valid_email(email) {
            return Err(AppError::InvalidEmail(email.to_owned()));
        }

        if utils::is_not_valid_phone_number(phone_number) {
            return Err(AppError::InvalidPhoneNumber(phone_number.to_owned()));
        }

        Ok(Self {
            first_name: first_name.to_owned(),
            last_name: last_name.to_owned(),
            display_name,
            email: email.to_owned(),
            phone_number: phone_number.to_owned(),
        })
    }
}
#[cfg(test)]
mod tests {
    use crate::utils::AppError;

    use super::{Construct, Contact};

    #[test]
    fn test_display_name() {
        let person = Contact::new("Jason", "Ribble", "john@example.com", "123-456-7890");
        let display_name = "Jason Ribble".to_string();

        assert_eq!(person.unwrap().display_name, display_name)
    }

    #[test]
    fn test_contact_update_construct() {
        let edits = Construct::new(
            1,
            None,
            None,
            None,
            Some("123-233-1221".to_string()),
            Some("Nickname".to_string()),
        )
        .unwrap();

        assert_eq!(edits.id, 1);
        assert_eq!(edits.update.display_name, Some("Nickname".to_string()));
        assert_eq!(edits.update.phone_number, Some("123-233-1221".to_string()));
        assert_eq!(edits.update.first_name, None);
        assert_eq!(edits.update.last_name, None);
        assert_eq!(edits.update.email, None);
    }

    #[test]
    fn test_contact_update_construct_2() {
        let edits = Construct::new(
            2,
            Some("Mary".to_string()),
            Some("Smith".to_string()),
            Some("new@email.com".to_string()),
            None,
            None,
        )
        .unwrap();

        assert_eq!(edits.id, 2);
        assert_eq!(edits.update.first_name, Some("Mary".to_string()));
        assert_eq!(edits.update.last_name, Some("Smith".to_string()));
        assert_eq!(edits.update.email, Some("new@email.com".to_string()));
        assert_eq!(edits.update.phone_number, None);
        assert_eq!(edits.update.display_name, None);
    }

    #[test]
    fn test_is_empty() {
        let result = Construct::new(1, None, None, None, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_email_construct() {
        let result = Construct::new(
            1,
            None,
            None,
            Some("invalid@example".to_string()),
            None,
            None,
        );
        assert!(result.is_err());
        assert!(matches!(result, Err(AppError::InvalidEmail(email)) if email == "invalid@example"));
    }

    #[test]
    fn test_invalid_construct_phone_number() {
        let result = Construct::new(1, None, None, None, Some("123-123-12345".to_string()), None);

        println!("{result:?}");
        assert!(result.is_err());

        assert!(
            matches!(result, Err(AppError::InvalidPhoneNumber(phone_number)) if phone_number == "123-123-12345")
        );
    }
}
