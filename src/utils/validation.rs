use regex::Regex;

fn is_valid_phone_number(phone: &str) -> bool {
    let phone_pattern =
        r"^\+?1?\s*(\(\d{3}\)|\d{3})[-.\s]*\d{3}[-.\s]*\d{4}(?:\s*(?:ext|x|ex)\.?\s*\d+)?$";
    let phone_regex = Regex::new(phone_pattern).unwrap();
    phone_regex.is_match(phone)
}

#[must_use]
pub fn is_not_valid_phone_number(phone_number: &str) -> bool {
    !is_valid_phone_number(phone_number)
}

fn is_valid_email(email: &str) -> bool {
    let email_pattern = r"^[\w\d][-\w\d+.]*@((?:[-\w\d]+\.)+[-\w\d]{2,})$";
    let email_regex = regex::Regex::new(email_pattern).unwrap();
    email_regex.is_match(email)
}

#[must_use]
pub fn is_not_valid_email(email: &str) -> bool {
    !is_valid_email(email)
}

#[cfg(test)]
mod tests {
    use crate::utils::{is_not_valid_email, is_not_valid_phone_number};

    use super::{is_valid_email, is_valid_phone_number};

    #[test]
    fn test_valid_phone_numbers() {
        let valid_numbers = [
            "1234567890",
            "123-456-7890",
            "123.456.7890",
            "(123) 456-7890",
            "+1 (123) 456-7890",
            "123-456-7890 ext. 1234",
            "123.456.7890 x1234",
            "(123) 456-7890 ex 1234",
            "1234567890x123",
        ];

        for number in &valid_numbers {
            assert!(
                is_valid_phone_number(number),
                "Valid number '{}' failed validation",
                number
            );
        }
    }

    #[test]
    fn test_invalid_phone_numbers() {
        let invalid_numbers = [
            "123456789",
            "1234567890123",
            "123-456-789",
            "(123 456-7890",
            "123.456.78901",
            "123-456-7890 ext",
            "123.456.7890 x",
            "(123) 456-7890 ex abc",
        ];

        for number in &invalid_numbers {
            assert!(
                is_not_valid_phone_number(number),
                "Invalid number '{}' passed validation",
                number
            );
        }
    }

    #[test]
    fn test_valid_email() {
        let valid_emails = [
            "john@example.com",
            "john.doe@example.com",
            "john_doe@example.co.uk",
            "john123@example.com",
            "john.doe+newsletter@example.com",
            "a@meow.com",
        ];

        for valid_email in valid_emails {
            assert!(
                is_valid_email(valid_email),
                "Valid email '{}' passed validation",
                valid_email
            )
        }
    }

    #[test]
    fn test_invalid_email() {
        let invalid_emails = [
            "john@example",
            "john.example.com",
            "john@.com",
            "john@example..com",
            "john@example.c",
            "john@example.com.",
            "john doe@example.com",
            " lohn doe@example.com",
            "john@example.com!",
            "jaasdf asdf @.com",
            ".test@example.com",
            "+test@example.com",
        ];

        for invalid_email in invalid_emails {
            assert!(
                is_not_valid_email(invalid_email),
                "Invalid email '{}' passed validation",
                invalid_email
            )
        }
    }
}
