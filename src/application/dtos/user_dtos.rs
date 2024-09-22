use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::entities::user::User;

// Data Tranfer Object for Creating User
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserDto {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDto {
    pub id: String,
    pub username: String,
    pub email: String,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        UserDto {
            id: user.id.to_string(),
            username: user.username,
            email: user.email,
            is_active: user.is_active,
            created_at: user.created_at.to_string(),
            updated_at: user.updated_at.to_string(),
        }
    }
}
