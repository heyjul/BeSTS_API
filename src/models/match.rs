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

#[derive(sqlx::FromRow)]
pub struct FullMatch {
    pub id: i64,
    pub team_one: String,
    pub team_two: String,
    pub start_date: DateTime<Utc>,
    pub winner_points: i64,
    pub guess_points: i64,
    pub guessed_team_one_score: Option<i64>,
    pub guessed_team_two_score: Option<i64>,
    pub real_team_one_score: Option<i64>,
    pub real_team_two_score: Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchDto {
    pub id: String,
    pub team_one: String,
    pub team_two: String,
    pub start_date: DateTime<Utc>,
    pub winner_points: i64,
    pub guess_points: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullMatchDto {
    pub id: String,
    pub team_one: String,
    pub team_two: String,
    pub start_date: DateTime<Utc>,
    pub winner_points: i64,
    pub guess_points: i64,
    pub guessed_team_one_score: Option<i64>,
    pub guessed_team_two_score: Option<i64>,
    pub real_team_one_score: Option<i64>,
    pub real_team_two_score: Option<i64>,
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

impl From<Match> for FullMatch {
    fn from(value: Match) -> Self {
        Self {
            id: value.id,
            team_one: value.team_one,
            team_two: value.team_two,
            start_date: value.start_date,
            winner_points: value.winner_points,
            guess_points: value.guess_points,
            guessed_team_one_score: None,
            guessed_team_two_score: None,
            real_team_one_score: None,
            real_team_two_score: None,
        }
    }
}

impl From<Match> for FullMatchDto {
    fn from(value: Match) -> Self {
        Self {
            id: encode_id(value.id),
            team_one: value.team_one,
            team_two: value.team_two,
            start_date: value.start_date,
            winner_points: value.winner_points,
            guess_points: value.guess_points,
            guessed_team_one_score: None,
            guessed_team_two_score: None,
            real_team_one_score: None,
            real_team_two_score: None,
        }
    }
}

impl From<FullMatch> for FullMatchDto {
    fn from(value: FullMatch) -> Self {
        Self {
            id: encode_id(value.id),
            team_one: value.team_one,
            team_two: value.team_two,
            start_date: value.start_date,
            winner_points: value.winner_points,
            guess_points: value.guess_points,
            guessed_team_one_score: value.guessed_team_one_score,
            guessed_team_two_score: value.guessed_team_two_score,
            real_team_one_score: value.real_team_one_score,
            real_team_two_score: value.real_team_two_score,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMatchRequestDto {
    pub id: Option<String>,
    pub team_one_id: String,
    pub team_two_id: String,
    pub start_date: DateTime<Utc>,
    pub winner_points: i64,
    pub guess_points: i64,
    pub room_id: String,
}

pub struct CreateMatchRequest {
    pub id: Option<i64>,
    pub team_one_id: i64,
    pub team_two_id: i64,
    pub start_date: DateTime<Utc>,
    pub winner_points: i64,
    pub guess_points: i64,
    pub room_id: i64,
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
            room_id: decode_id(value.room_id)?,
        })
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CloseMatchRequestDto {
    pub team_one_score: i64,
    pub team_two_score: i64,
}

pub struct CloseMatchRequest {
    pub team_one_score: i64,
    pub team_two_score: i64,
}

impl From<CloseMatchRequestDto> for CloseMatchRequest {
    fn from(value: CloseMatchRequestDto) -> Self {
        Self {
            team_one_score: value.team_one_score,
            team_two_score: value.team_two_score,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchResultDto {
    pub match_id: String,
    pub team_one_score: i64,
    pub team_two_score: i64,
}

pub struct MatchResult {
    pub match_id: i64,
    pub team_one_score: i64,
    pub team_two_score: i64,
}

impl From<MatchResult> for MatchResultDto {
    fn from(value: MatchResult) -> Self {
        Self {
            match_id: encode_id(value.match_id),
            team_one_score: value.team_one_score,
            team_two_score: value.team_two_score,
        }
    }
}
