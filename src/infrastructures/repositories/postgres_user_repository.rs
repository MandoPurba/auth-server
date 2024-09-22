use crate::domain::entities::user::User;
use crate::domain::repositories::user_repository::UserRepository;
use sqlx::{Error, Pool, Postgres};
use uuid::Uuid;

pub struct PgUserRepository {
    pub pool: Pool<Postgres>,
}

#[async_trait::async_trait]
impl UserRepository for PgUserRepository {
    async fn create_user(
        &self,
        username: &str,
        email: &str,
        password_hash: &str,
    ) -> Result<User, Error> {
        match sqlx::query_as::<_, User>("--sql
            INSERT INTO users (id, username, email, password_hash, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, username, email, password_hash, is_active, created_at, updated_at
        ")
        .bind(Uuid::new_v4())
        .bind(username)
        .bind(email)
        .bind(password_hash)
        .bind(true)
        .bind(chrono::Utc::now())
        .bind(chrono::Utc::now())
        .fetch_one(&self.pool)
        .await {
            Ok(user) => Ok(user),
            Err(e) => Err(e),
        }
    }

    async fn find_user_by_username(&self, username: &str) -> Result<Option<User>, Error> {
        match sqlx::query_as::<_, User>(
            "--sql
            SELECT id, username, email. password_hash, is_active, created_at, updated_at FROM users
            WHERE username = $1
        ",
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
        {
            Ok(user) => Ok(user),
            Err(e) => Err(e),
        }
    }

    async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, Error> {
        match sqlx::query_as::<_, User>(
            "--sql
            SELECT id, username, email. password_hash, is_active, created_at, updated_at FROM users
            WHERE email = $1
        ",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        {
            Ok(user) => Ok(user),
            Err(e) => Err(e),
        }
    }

    async fn update_user(&self, user: &User) -> Result<(), Error> {
        todo!()
    }

    async fn delete_user(&self, user_id: &str) -> Result<(), Error> {
        todo!()
    }
}
