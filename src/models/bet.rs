use serde::{Deserialize, Serialize};

use crate::utils::hasher::encode_id;

pub struct Bet {
    pub match_id: i64,
    pub user_id: i64,
    pub team_one_score: i64,
    pub team_two_score: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BetDto {
    pub match_id: String,
    pub user_id: String,
    pub team_one_score: i64,
    pub team_two_score: i64,
}

impl From<Bet> for BetDto {
    fn from(value: Bet) -> Self {
        Self {
            match_id: encode_id(value.match_id),
            user_id: encode_id(value.user_id),
            team_one_score: value.team_one_score,
            team_two_score: value.team_two_score,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBetRequestDto {
    pub team_one_score: i64,
    pub team_two_score: i64,
}

pub struct CreateBetRequest {
    pub team_one_score: i64,
    pub team_two_score: i64,
}

impl From<CreateBetRequestDto> for CreateBetRequest {
    fn from(value: CreateBetRequestDto) -> Self {
        Self {
            team_one_score: value.team_one_score,
            team_two_score: value.team_two_score,
        }
    }
}

pub struct FullBet {
    pub username: String,
    pub team_one: String,
    pub team_two: String,
    pub team_one_score: i64,
    pub team_two_score: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullBetDto {
    pub username: String,
    pub team_one: String,
    pub team_two: String,
    pub team_one_score: i64,
    pub team_two_score: i64,
}

impl From<FullBet> for FullBetDto {
    fn from(value: FullBet) -> Self {
        Self {
            username: value.username,
            team_one: value.team_one,
            team_two: value.team_two,
            team_one_score: value.team_one_score,
            team_two_score: value.team_two_score,
        }
    }
}
