use sqlx::Error;

use crate::domain::entities::user::User;

#[async_trait::async_trait]
pub trait UserRepository {
    async fn create_user(
        &self,
        username: &str,
        email: &str,
        password_hash: &str,
    ) -> Result<User, Error>;
    async fn find_user_by_username(&self, username: &str) -> Result<Option<User>, Error>;
    async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, Error>;
    async fn update_user(&self, user: &User) -> Result<(), Error>;
    async fn delete_user(&self, user_id: &str) -> Result<(), Error>;
}
