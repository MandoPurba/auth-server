use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Authentication error: {0}")]
    Authentication(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("External service error: {0}")]
    ExternalService(String),

    #[error("Unexpected error: {0}")]
    Unexpected(String),
}

impl From<sqlx::Error> for ApplicationError {
    fn from(err: sqlx::Error) -> Self {
        ApplicationError::Database(err.to_string())
    }
}

impl From<argon2::password_hash::Error> for ApplicationError {
    fn from(err: argon2::password_hash::Error) -> Self {
        ApplicationError::Authentication(format!("Password hashing error: {}", err))
    }
}
