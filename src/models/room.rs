use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Room {
    pub id: i64,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateRoomRequest {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct RoomWithUrl {
    pub name: String,
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct FullRoom {
    pub id: i64,
    pub name: String,
    pub owner: i64,
}
