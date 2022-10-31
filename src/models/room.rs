use serde::{Deserialize, Serialize};

use crate::utils::hasher::encode_id;

use super::r#match::FullMatchDto;

pub struct Room {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub owner_id: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRoomRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomDto {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub owner_id: String,
}

impl From<Room> for RoomDto {
    fn from(value: Room) -> Self {
        Self {
            id: encode_id(value.id),
            name: value.name,
            description: value.description,
            owner_id: encode_id(value.owner_id),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullRoomDto {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub owner_id: String,
    pub matches: Vec<FullMatchDto>,
}
