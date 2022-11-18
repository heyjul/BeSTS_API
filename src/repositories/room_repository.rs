use sqlx::{Pool, Sqlite};

use crate::models::{
    error::{Error, Errors},
    room::{CreateRoomRequest, Room},
    score::Score,
};

use super::factory::RepositoryFactory;

pub struct RoomRepository {
    db_pool: Pool<Sqlite>,
}

impl RepositoryFactory for RoomRepository {
    fn new(db_pool: Pool<Sqlite>) -> Self {
        Self { db_pool }
    }
}

impl RoomRepository {
    pub async fn get_rooms(&self, user_id: i64) -> Error<Vec<Room>> {
        let rooms = sqlx::query_as!(
            Room,
            "
            SELECT DISTINCT
                room.id,
                room.name,
                room.description,
                room.owner_id
            FROM
                room 
                LEFT JOIN room_user ON room.id = room_user.room_id
            WHERE
                room.deleted_date IS NULL
                AND (
                    room_user.user_id = ?
                    OR room.owner_id = ?
                )
            ",
            user_id,
            user_id
        )
        .fetch_all(&self.db_pool)
        .await?;

        Ok(rooms)
    }

    pub async fn get_room(&self, id: i64) -> Error<Option<Room>> {
        let room = sqlx::query_as!(
            Room,
            "
            SELECT
                id,
                name,
                description,
                owner_id
            FROM
                room
            WHERE
                id = ?
                AND deleted_date IS NULL
            ",
            id
        )
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(room)
    }

    pub async fn create(&self, room: CreateRoomRequest, user_id: i64) -> Error<Room> {
        let inserted_room = sqlx::query_as!(
            Room,
            "
            INSERT INTO room
                (name, description, owner_id)
            VALUES
                (?, ?, ?);

            SELECT
                id,
                name,
                description,
                owner_id
            FROM
                room
            WHERE
                rowid = last_insert_rowid()
            ",
            room.name,
            room.description,
            user_id
        )
        .fetch_one(&self.db_pool)
        .await?;

        Ok(inserted_room)
    }

    pub async fn join(&self, id: i64, user_id: i64) -> Error<Room> {
        let present = sqlx::query!(
            "
            SELECT CASE 
                WHEN EXISTS (SELECT *
                            FROM
                                room 
                                JOIN room_user ON room.id = room_user.room_id
                            WHERE
                                room.id = ?
                                AND (room_user.user_id = ?
                                    OR room.owner_id = ?))
                    THEN 1
                ELSE 0
            END present
            ",
            id,
            user_id,
            user_id
        )
        .fetch_one(&self.db_pool)
        .await?
        .present;

        if present == 1 {
            return Err(Box::new(Errors::AlreadyJoined(
                "User has already joined this room.",
            )));
        }

        let room = self
            .get_room(id)
            .await?
            .ok_or(Errors::NotFound("Room not found."))?;

        sqlx::query!(
            "
            INSERT INTO room_user
                (room_id, user_id)
            VALUES
                (?, ?)
            ",
            id,
            user_id
        )
        .execute(&self.db_pool)
        .await?;

        Ok(room)
    }

    pub async fn delete(&self, id: i64, user_id: i64) -> Error<()> {
        let room = self
            .get_room(id)
            .await?
            .ok_or(Errors::NotFound("Room not found"))?;

        if room.owner_id != user_id {
            return Err(Box::new(Errors::NotAllowed(
                "You must be the owner to delete this room",
            )));
        }

        let now = chrono::Local::now();
        sqlx::query!(
            "
            UPDATE
                room
            SET
                deleted_date = ?,
                deleted_by = ?
            WHERE
                id = ?;
            ",
            now,
            user_id,
            id
        )
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }

    pub async fn get_scores(&self, id: i64) -> Error<Vec<Score>> {
        let scores = sqlx::query_as::<_, Score>(
            "
            SELECT
                user.username,
                score.score
            FROM
                score
                JOIN user ON score.user_id = user.id
            WHERE
                score.room_id = ?
            ",
        )
        .bind(id)
        .fetch_all(&self.db_pool)
        .await?;

        Ok(scores)
    }
}
