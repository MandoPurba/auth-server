use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString
    },
    Argon2
};

use crate::{
    application::dtos::user_dtos::{CreateUserDto, UserDto},
    domain::repositories::user_repository::UserRepository,
    application::errors::ApplicationError,
};

pub struct RegisterUserUseCase<R: UserRepository> {
    pub repository: R,
}

impl<R: UserRepository> RegisterUserUseCase<R> {
    pub async fn execute(&self, input: CreateUserDto) -> Result<UserDto, ApplicationError> {
        // Hash the password before saving
        let password_hash = hash_password(&input.password)?;

        // Create a new user using the repository
        let user = self
            .repository
            .create_user(&input.username, &input.email, &password_hash)
            .await?;

        // Convert the domain entity to DTO and return it
        Ok(UserDto::from(user))
    }
}

// A proper password hashing function using Argon2
fn hash_password(password: &str) -> Result<String, ApplicationError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2.hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(ApplicationError::from)
}
