mod errors;
mod validation;

pub use errors::AppError;
pub use validation::{is_not_valid_email, is_not_valid_phone_number};
