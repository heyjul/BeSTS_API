use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow)]
pub struct Score {
    pub username: String,
    pub score: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScoreDto {
    pub username: String,
    pub score: i64,
}

impl From<Score> for ScoreDto {
    fn from(value: Score) -> Self {
        Self {
            username: value.username,
            score: value.score,
        }
    }
}
