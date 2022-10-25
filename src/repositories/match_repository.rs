use sqlx::{Pool, Sqlite};

use crate::models::r#match::{CreateMatchRequest, Match};

use super::factory::RepositoryFactory;

pub struct MatchRepository {
    db_pool: Pool<Sqlite>,
}

impl RepositoryFactory for MatchRepository {
    fn new(db_pool: Pool<Sqlite>) -> Self {
        Self { db_pool }
    }
}

impl MatchRepository {
    pub async fn get(&self, room_id: i64) -> Result<Vec<Match>, Box<dyn std::error::Error>> {
        let matches = sqlx::query_as!(
            Match,
            r#"
            SELECT
                match.id,
                team1.name AS team_one,
                team2.name AS team_two,
                match.start_date AS "start_date: _",
                match.winner_points,
                match.guess_points
            FROM
                match
                JOIN team as team1 on match.team_one_id = team1.id
                JOIN team as team2 on match.team_two_id = team2.id
            WHERE
                room_id = ?
            "#,
            room_id
        )
        .fetch_all(&self.db_pool)
        .await?;

        Ok(matches)
    }

    pub async fn create(
        &self,
        req: CreateMatchRequest,
    ) -> Result<Match, Box<dyn std::error::Error>> {
        let r#match = sqlx::query_as!(
            Match,
            r#"
            INSERT INTO match
                (team_one_id, team_two_id, start_date, winner_points, guess_points, room_id)
            VALUES
                (?, ?, ?, ?, ?, ?);

            SELECT
                match.id,
                team1.name AS team_one,
                team2.name AS team_two,
                match.start_date AS "start_date: _",
                match.winner_points,
                match.guess_points
            FROM
                match
                JOIN team as team1 on match.team_one_id = team1.id
                JOIN team as team2 on match.team_two_id = team2.id
            WHERE
                match.rowid = last_insert_rowid();
            "#,
            req.team_one_id,
            req.team_two_id,
            req.start_date,
            req.winner_points,
            req.guess_points,
            req.room_id
        )
        .fetch_one(&self.db_pool)
        .await?;

        Ok(r#match)
    }

    pub async fn update(
        &self,
        req: CreateMatchRequest,
    ) -> Result<Match, Box<dyn std::error::Error>> {
        let r#match = sqlx::query_as!(
            Match,
            r#"
            UPDATE match SET 
                team_one_id = ?, 
                team_two_id = ?, 
                start_date = ?, 
                winner_points = ?, 
                guess_points = ?
            WHERE   
                id = ?;

            SELECT
                match.id,
                team1.name AS team_one,
                team2.name AS team_two,
                match.start_date AS "start_date: _",
                match.winner_points,
                match.guess_points
            FROM
                match
                JOIN team as team1 on match.team_one_id = team1.id
                JOIN team as team2 on match.team_two_id = team2.id
            WHERE
                match.id = ?;
            "#,
            req.team_one_id,
            req.team_two_id,
            req.start_date,
            req.winner_points,
            req.guess_points,
            req.id,
            req.id,
        )
        .fetch_one(&self.db_pool)
        .await?;

        Ok(r#match)
    }
}
