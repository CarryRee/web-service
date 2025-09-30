use crate::models::user::User;
use crate::repositories::user_repo::UserRepo;
use async_trait::async_trait;
use sqlx::PgPool;

pub struct PgUserRepo {
    pool: PgPool,
}

impl PgUserRepo {
    pub fn new(pool: PgPool) -> Self {
        PgUserRepo { pool }
    }
}

#[async_trait]
impl UserRepo for PgUserRepo {
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT id, username, email, password_hash, created_at, updated_at, is_active, role FROM users WHERE username = $1")
            .bind(username)
            .fetch_optional(&self.pool)
            .await
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT id, username, email, password_hash, created_at, updated_at, is_active, role FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(&self.pool)
            .await
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT id, username, email, password_hash, created_at, updated_at, is_active, role FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    async fn create(
        &self,
        id: i64,
        username: String,
        email: String,
        password_hash: String,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO users (id, username, email, password_hash) VALUES ($1, $2, $3, $4)",
        )
        .bind(id)
        .bind(username)
        .bind(email)
        .bind(password_hash)
        .execute(&self.pool)
        .await
        .map(|_| ())
    }
}
