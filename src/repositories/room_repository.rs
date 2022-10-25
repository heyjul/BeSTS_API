use sqlx::{Pool, Sqlite};

use crate::models::{
    room::{CreateRoomRequest, Room},
    room_error::RoomError,
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
    pub async fn get_rooms(&self, user_id: i64) -> Result<Vec<Room>, Box<dyn std::error::Error>> {
        let rooms = sqlx::query_as!(
            Room,
            "
            SELECT
                room.id,
                room.name,
                room.owner_id
            FROM
                room_user
                JOIN room ON room_user.room_id = room.id
            WHERE
                room_user.user_id = ?
                OR room.owner_id = ?
            ",
            user_id,
            user_id
        )
        .fetch_all(&self.db_pool)
        .await?;

        Ok(rooms)
    }

    pub async fn get_room(&self, id: i64) -> Result<Option<Room>, Box<dyn std::error::Error>> {
        let room = sqlx::query_as!(
            Room,
            "
            SELECT
                id,
                name,
                owner_id
            FROM
                room
            WHERE
                id = ?
            ",
            id
        )
        .fetch_optional(&self.db_pool)
        .await?;

        Ok(room)
    }

    pub async fn create(
        &self,
        room: CreateRoomRequest,
        user_id: i64,
    ) -> Result<Room, Box<dyn std::error::Error>> {
        let inserted_room = sqlx::query_as!(
            Room,
            "
            INSERT INTO room
                (name, owner_id)
            VALUES
                (?, ?);

            SELECT
                id,
                name,
                owner_id
            FROM
                room
            WHERE
                rowid = last_insert_rowid()
            ",
            room.name,
            user_id
        )
        .fetch_one(&self.db_pool)
        .await?;

        Ok(inserted_room)
    }

    pub async fn join(&self, id: i64, user_id: i64) -> Result<Room, Box<dyn std::error::Error>> {
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
            return Err(RoomError::AlreadyJoined(()))?;
        }

        let room = self
            .get_room(id)
            .await?
            .ok_or(RoomError::RoomNotFound(()))?;

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

        Ok(Room {
            id: room.id,
            name: room.name,
            owner_id: room.owner_id,
        })
    }

    pub async fn delete(&self, id: i64, user_id: i64) -> Result<(), Box<dyn std::error::Error>> {
        let room = self
            .get_room(id)
            .await?
            .ok_or(RoomError::RoomNotFound(()))?;

        if room.owner_id != user_id {
            return Err(RoomError::NotAllowed(()))?;
        }

        sqlx::query!(
            "
            DELETE FROM room_user WHERE room_id = ?;
            DELETE FROM room WHERE  id = ?
            ",
            id,
            id
        )
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }
}
