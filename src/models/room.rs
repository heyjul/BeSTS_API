use serde::{Deserialize, Serialize};

use crate::utils::hasher::encode_id;

pub struct Room {
    pub id: i64,
    pub name: String,
    pub owner_id: i64,
}

#[derive(Serialize, Deserialize)]
pub struct CreateRoomRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct RoomDto {
    pub id: String,
    pub name: String,
}

impl From<Room> for RoomDto {
    fn from(value: Room) -> Self {
        Self {
            id: encode_id(value.id),
            name: value.name,
        }
    }
}
