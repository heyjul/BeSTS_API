use std::error::Error;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::utils::hasher::{decode_id, encode_id};

pub struct Match {
    pub id: i64,
    pub team_one: String,
    pub team_two: String,
    pub start_date: DateTime<Utc>,
    pub winner_points: i64,
    pub guess_points: i64,
}

#[derive(Serialize, Deserialize)]
pub struct MatchDto {
    pub id: String,
    pub team_one: String,
    pub team_two: String,
    pub start_date: DateTime<Utc>,
    pub winner_points: i64,
    pub guess_points: i64,
}

impl From<Match> for MatchDto {
    fn from(value: Match) -> Self {
        Self {
            id: encode_id(value.id),
            team_one: value.team_one,
            team_two: value.team_two,
            start_date: value.start_date,
            winner_points: value.winner_points,
            guess_points: value.guess_points,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateMatchRequestDto {
    pub id: Option<String>,
    pub team_one_id: String,
    pub team_two_id: String,
    pub start_date: DateTime<Utc>,
    pub winner_points: i64,
    pub guess_points: i64,
    pub room_id: Option<String>,
}

pub struct CreateMatchRequest {
    pub id: Option<i64>,
    pub team_one_id: i64,
    pub team_two_id: i64,
    pub start_date: DateTime<Utc>,
    pub winner_points: i64,
    pub guess_points: i64,
    pub room_id: Option<i64>,
}

impl TryFrom<CreateMatchRequestDto> for CreateMatchRequest {
    type Error = Box<dyn Error>;

    fn try_from(value: CreateMatchRequestDto) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id.map(decode_id).transpose()?,
            team_one_id: decode_id(value.team_one_id)?,
            team_two_id: decode_id(value.team_two_id)?,
            start_date: value.start_date,
            winner_points: value.winner_points,
            guess_points: value.guess_points,
            room_id: value.room_id.map(decode_id).transpose()?,
        })
    }
}
