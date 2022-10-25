use serde::{Deserialize, Serialize};

pub struct Team {
    pub id: i64,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct TeamDto {
    pub id: String,
    pub name: String,
}
