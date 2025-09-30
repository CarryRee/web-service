use crate::models::user::User;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepo: Send + Sync  {
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, sqlx::Error>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error>;
    async fn find_by_id(&self, id: i32) -> Result<Option<User>, sqlx::Error>;
    async fn create(&self, id: i64, username: String, email: String, password_hash: String) -> Result<(), sqlx::Error>;
}
