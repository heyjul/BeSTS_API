use sqlx::{Pool, Sqlite};

use crate::models::{
    room::{CreateRoomRequest, FullRoom, Room},
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
                rooms.id,
                rooms.name
            FROM
                rooms_users
                JOIN rooms ON rooms_users.room_id = rooms.id
            WHERE
                rooms_users.user_id = ?
                OR rooms.owner = ?
            ",
            user_id,
            user_id
        )
        .fetch_all(&self.db_pool)
        .await?;

        Ok(rooms)
    }

    pub async fn get_room(&self, id: i64) -> Result<Option<FullRoom>, Box<dyn std::error::Error>> {
        let room = sqlx::query_as!(
            FullRoom,
            "
            SELECT
                id,
                name,
                owner
            FROM
                rooms
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
            INSERT INTO rooms
                (name, owner)
            VALUES
                (?, ?);

            SELECT
                id,
                name
            FROM
                rooms
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
                                rooms 
                                JOIN rooms_users ON rooms.id = rooms_users.room_id
                            WHERE
                                rooms.id = ?
                                AND (rooms_users.user_id = ?
                                    OR rooms.owner = ?))
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
            INSERT INTO rooms_users
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
        })
    }

    pub async fn delete(&self, id: i64, user_id: i64) -> Result<(), Box<dyn std::error::Error>> {
        let room = self
            .get_room(id)
            .await?
            .ok_or(RoomError::RoomNotFound(()))?;

        if room.owner != user_id {
            return Err(RoomError::NotAllowed(()))?;
        }

        sqlx::query!(
            "
            DELETE FROM rooms_users WHERE room_id = ?;
            DELETE FROM rooms WHERE  id = ?
            ",
            id,
            id
        )
        .execute(&self.db_pool)
        .await?;

        Ok(())
    }
}
