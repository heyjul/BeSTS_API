use std::error::Error;

use sqlx::{Pool, Sqlite};

use crate::models::bet::{Bet, CreateBetRequest, FullBet};

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
                ((SELECT id FROM match where start_date > DATETIME() and id = ?), ?, ?, ?);

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
                AND match_id = (SELECT id FROM match WHERE start_date > DATETIME() AND id = ?);

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

    pub async fn get_by_match(&self, match_id: i64) -> Result<Vec<FullBet>, Box<dyn Error>> {
        let bets = sqlx::query_as!(
            FullBet,
            "
            SELECT
                user.username AS username,
                t1.name AS team_one,
                t2.name AS team_two,
                bet.team_one_score AS team_one_score,
                bet.team_two_score AS team_two_score
            FROM
                bet
                JOIN user ON bet.user_id = user.id 
                JOIN match ON bet.match_id = match.id
                JOIN team t1 ON match.team_one_id = t1.id
                JOIN team t2 ON match.team_two_id = t2.id
            WHERE
                bet.match_id = ?;
            ",
            match_id,
        )
        .fetch_all(&self.db_pool)
        .await?;

        Ok(bets)
    }
}
