use std::error::Error;

use sqlx::{Pool, Sqlite};

use crate::models::bet::{Bet, CreateBetRequest};

use super::factory::RepositoryFactory;

pub struct BetRepository {
    db_pool: Pool<Sqlite>,
}

impl RepositoryFactory for BetRepository {
    fn new(db_pool: Pool<Sqlite>) -> Self {
        Self { db_pool }
    }
}

impl BetRepository {
    pub async fn create(
        &self,
        req: CreateBetRequest,
        match_id: i64,
        user_id: i64,
    ) -> Result<Bet, Box<dyn Error>> {
        let bet = sqlx::query_as!(
            Bet,
            "
            INSERT INTO bet
                (match_id, user_id, team_one_score, team_two_score)
            VALUES
                (?, ?, ?, ?);

            SELECT
                match_id,
                user_id,
                team_one_score,
                team_two_score
            FROM
                bet
            WHERE
                rowid = last_insert_rowid();
            ",
            match_id,
            user_id,
            req.team_one_score,
            req.team_two_score,
        )
        .fetch_one(&self.db_pool)
        .await?;

        Ok(bet)
    }

    pub async fn update(
        &self,
        req: CreateBetRequest,
        match_id: i64,
        user_id: i64,
    ) -> Result<Bet, Box<dyn Error>> {
        let bet = sqlx::query_as!(
            Bet,
            "
            UPDATE bet SET
                team_one_score = ?, 
                team_two_score = ?
            WHERE
                user_id = ?
                AND match_id = ?;

            SELECT
                match_id,
                user_id,
                team_one_score,
                team_two_score
            FROM
                bet
            WHERE
                user_id = ?
                AND match_id = ?;
            ",
            req.team_one_score,
            req.team_two_score,
            user_id,
            match_id,
            user_id,
            match_id,
        )
        .fetch_one(&self.db_pool)
        .await?;

        Ok(bet)
    }

    pub async fn get_by_id(
        &self,
        match_id: i64,
        user_id: i64,
    ) -> Result<Option<Bet>, Box<dyn Error>> {
        let bet = sqlx::query_as!(
            Bet,
            "
            SELECT
                match_id,
                user_id,
                team_one_score,
                team_two_score
            FROM
                bet
            WHERE
                user_id = ?
                AND match_id = ?;
            ",
            user_id,
            match_id,
        )
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(bet)
    }
}
