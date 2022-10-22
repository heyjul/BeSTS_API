use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Room {
    pub id: i64,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct FullRoom {
    pub id: i64,
    pub name: String,
    pub owner: i64,
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
