use sqlx::{Pool, Sqlite};

use crate::models::r#match::{CreateMatchRequest, FullMatch, Match};

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
    pub async fn get(
        &self,
        room_id: i64,
        user_id: i64,
    ) -> Result<Vec<FullMatch>, Box<dyn std::error::Error>> {
        let matches = sqlx::query_as::<_, FullMatch>(
            r#"
            SELECT
                match.id,
                team1.name AS team_one,
                team2.name AS team_two,
                match.start_date AS start_date,
                match.winner_points,
                match.guess_points,
                bet.team_one_score AS guessed_team_one_score,
                bet.team_two_score AS guessed_team_two_score
            FROM
                match
                JOIN team as team1 on match.team_one_id = team1.id
                JOIN team as team2 on match.team_two_id = team2.id
                LEFT JOIN bet ON match.id = bet.match_id AND bet.user_id = ?
            WHERE
                room_id = ?
            ORDER BY
                match.start_date
            "#,
        )
        .bind(user_id)
        .bind(room_id)
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

    pub async fn is_allowed(
        &self,
        match_id: i64,
        user_id: i64,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let result = sqlx::query!(
            "
            SELECT
                match.id
            FROM
                match
                JOIN room on match.room_id = room.id
                LEFT JOIN room_user on room.id = room_user.room_id
            WHERE
                match.id = ?
                AND (
                    room.owner_id = ?
                    OR room_user.user_id = ?
                );
            ",
            match_id,
            user_id,
            user_id,
        )
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(result.is_some())
    }

    pub async fn delete(
        &self,
        match_id: i64,
        user_id: i64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query!(
            "
            DELETE FROM 
                match
            WHERE
                id = ?
                AND EXISTS ( 
                    SELECT
                        *
                    FROM 
                        match
                        JOIN room ON match.room_id = room.id
                    WHERE
                        room.owner_id = ?
                        AND match.id = ?
                    );
            ",
            match_id,
            user_id,
            match_id,
        )
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }
}
