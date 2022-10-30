use sqlx::{Pool, Sqlite};

use crate::models::team::Team;

use super::factory::RepositoryFactory;

pub struct TeamRepository {
    db_pool: Pool<Sqlite>,
}

impl RepositoryFactory for TeamRepository {
    fn new(db_pool: Pool<Sqlite>) -> Self {
        Self { db_pool }
    }
}

impl TeamRepository {
    pub async fn get(&self) -> Result<Vec<Team>, Box<dyn std::error::Error>> {
        let teams = sqlx::query_as!(
            Team,
            "
            SELECT
                id,
                name
            FROM
                team
            ORDER BY
                2
            "
        )
        .fetch_all(&self.db_pool)
        .await?;

        Ok(teams)
    }
}
