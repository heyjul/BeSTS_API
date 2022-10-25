use serde::{Deserialize, Serialize};

use crate::utils::hasher::encode_id;

pub struct Team {
    pub id: i64,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct TeamDto {
    pub id: String,
    pub name: String,
}

impl From<Team> for TeamDto {
    fn from(value: Team) -> Self {
        Self {
            id: encode_id(value.id),
            name: value.name,
        }
    }
}
