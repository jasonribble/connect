use std::fmt;

#[derive(Debug)]
pub enum AppError {
    DatabaseError(sqlx::Error),
    InvalidEmail(String),
    InvalidPhoneNumber(String),
    EmptyUpdate,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::DatabaseError(error) => write!(f, "Database error: {error}"),
            Self::InvalidEmail(email) => write!(f, "{email} is invalid."),
            Self::InvalidPhoneNumber(phone) => write!(f, "{phone} is invalid."),
            Self::EmptyUpdate => write!(f, "Empty update"),
        }
    }
}

impl std::error::Error for AppError {}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        Self::DatabaseError(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::Error as SqlxError;

    #[test]
    fn test_app_error_from_sqlx_error() {
        let sqlx_error = SqlxError::PoolTimedOut;
        let app_error = AppError::from(sqlx_error);
        assert!(matches!(app_error, AppError::DatabaseError(_)));
    }

    #[test]
    fn test_app_error_display() {
        let sqlx_error = SqlxError::PoolTimedOut;
        let app_error = AppError::from(sqlx_error);
        assert_eq!(
            format!("{}", app_error),
            "Database error: pool timed out while waiting for an open connection"
        );
    }

    #[test]
    fn test_app_error_debug() {
        let sqlx_error = SqlxError::PoolTimedOut;
        let app_error = AppError::from(sqlx_error);
        assert!(format!("{:?}", app_error).starts_with("DatabaseError"));
    }

    #[test]
    fn test_app_error_is_error() {
        let sqlx_error = SqlxError::PoolTimedOut;
        let app_error = AppError::from(sqlx_error);
        let _: Box<dyn std::error::Error> = Box::new(app_error);
    }

    #[test]
    fn test_invalid_email_error_message() {
        let invalid_email = "not-an-email".to_string();
        let app_error = AppError::InvalidEmail(invalid_email);
        assert_eq!(format!("{}", app_error), "not-an-email is invalid.");
    }

    #[test]
    fn test_invalid_phone_number_error_message() {
        let invalid_phone = "123".to_string();
        let app_error = AppError::InvalidPhoneNumber(invalid_phone);
        assert_eq!(format!("{}", app_error), "123 is invalid.");
    }
}
