use serde::{Deserialize, Serialize};

pub struct Room {
    pub id: i64,
    pub name: String,
}

pub struct FullRoom {
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
    pub id: Option<String>,
    pub name: String,
}
