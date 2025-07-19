use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use validator::{Validate, ValidationError};

pub struct AppState {
    pub db: MySqlPool,
}

#[derive(Deserialize, Debug, Validate)]
pub struct User {
    #[serde(rename = "user_name")]
    #[validate(email)]
    pub email: String,

    #[validate(custom(function = "validate_password"))]
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct Login {
    #[serde(rename = "user_name")]
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Debug)]
pub struct UserInDB {
    pub email: String,
    pub password: String,
}

pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    let has_uppercase = password.chars().any(|c| c.is_ascii_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_ascii_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_special = password.chars().any(|c| "!@#$%^&*".contains(c));

    if has_uppercase && has_lowercase && has_digit && has_special && password.len() >= 8 {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_password"))
    }
}
