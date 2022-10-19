use crate::{
    models::{auth::RegisterRequest, auth_error::AuthError, user::User},
    utils::password,
};
use sqlx::{Pool, Sqlite};

use super::factory::RepositoryFactory;

pub struct AuthRepository {
    db_pool: Pool<Sqlite>,
}

impl RepositoryFactory for AuthRepository {
    fn new(db_pool: Pool<Sqlite>) -> Self {
        Self { db_pool }
    }
}

impl AuthRepository {
    pub async fn register(&self, req: RegisterRequest) -> Result<bool, Box<dyn std::error::Error>> {
        let record = sqlx::query!(
            "  
    SELECT 
        CASE WHEN EXISTS (
            SELECT 
                * 
            FROM 
                users 
            WHERE 
                email = ?
        ) THEN 1 ELSE 0 end email_exists, 
        CASE WHEN EXISTS (
            SELECT 
                * 
            FROM 
                users 
            WHERE 
                username = ?
        ) THEN 1 ELSE 0 end username_exists
                ",
            req.email,
            req.username
        )
        .fetch_one(&self.db_pool)
        .await?;

        if record.email_exists == 1 {
            return Err(Box::new(AuthError::EmailTaken(req.email)));
        }

        if record.username_exists == 1 {
            return Err(Box::new(AuthError::UsernameTaken(req.username)));
        }

        let hashed_password = password::hash_password(&req.password)?;

        let x = sqlx::query!(
            "
            INSERT INTO users 
                (username, email, password)
            VALUES
                (?, ?, ?)
            ",
            req.username,
            req.email,
            hashed_password
        )
        .execute(&self.db_pool)
        .await?;

        Ok(x.rows_affected() == 1)
    }

    pub async fn get_user_by_email(
        &self,
        email: &str,
    ) -> Result<Option<User>, Box<dyn std::error::Error>> {
        let user = sqlx::query_as!(
            User,
            "
        SELECT
            id,
            username,
            email,
            password
        FROM
            users
        WHERE
            email = ?
            ",
            email
        )
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(user)
    }
}
