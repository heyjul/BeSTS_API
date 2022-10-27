use std::error::Error;

use rocket::{
    http::Status,
    outcome::{try_outcome, Outcome::*},
    request::{FromRequest, Outcome},
    Request,
};
use serde::{Deserialize, Serialize};

use crate::{
    repositories::{
        factory::Factory, match_repository::MatchRepository, room_repository::RoomRepository,
    },
    utils::{hasher::decode_id, jwt},
};

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Token {
    pub token: String,
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub struct User {
    pub id: i64,
    pub email: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for &'r User {
    type Error = &'r str;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(bearer) = request.headers().get_one("Authorization") {
            if let Some((_, token)) = bearer.split_once(' ') {
                let claims = match jwt::verify(token) {
                    Ok(claims) => claims,
                    Err(_) => return Failure((Status::Unauthorized, "Cannot parse jwt token")),
                };

                let user = match decode_id(claims.sub) {
                    Ok(id) => User {
                        id,
                        email: claims.email,
                    },
                    Err(_) => return Failure((Status::Unauthorized, "Cannot parse jwt token")),
                };

                return Success(request.local_cache(move || user));
            }
        }

        Failure((Status::Unauthorized, "You must be connected"))
    }
}

pub struct RoomUser {
    pub id: i64,
    pub email: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for &'r RoomUser {
    type Error = &'r str;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user = try_outcome!(request.guard::<&User>().await);

        let room_id = match get_from_request(request, "room_id") {
            Ok(x) => x,
            Err(_) => {
                return Failure((Status::Unauthorized, "Cannot find the room id in the path"))
            }
        };

        let room_id = match decode_id(room_id.to_owned()) {
            Ok(room_id) => room_id,
            Err(_) => {
                return Failure((
                    Status::InternalServerError,
                    "Cannot parse the requested room id",
                ))
            }
        };

        let factory = try_outcome!(request.guard::<&Factory>().await);

        match factory.get::<RoomRepository>().get_rooms(user.id).await {
            Ok(rooms) => {
                if rooms.iter().any(|x| x.id == room_id) {
                    Success(request.local_cache(move || RoomUser {
                        id: user.id,
                        email: user.email.clone(),
                    }))
                } else {
                    Failure((Status::Unauthorized, "You do not have access to this room"))
                }
            }
            Err(_) => Failure((Status::Unauthorized, "Cannot get user's rooms")),
        }
    }
}

pub struct MatchUser {
    pub id: i64,
    pub email: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for &'r MatchUser {
    type Error = &'r str;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user = try_outcome!(request.guard::<&User>().await);

        let match_id = match get_from_request(request, "match_id") {
            Ok(x) => x,
            Err(_) => {
                return Failure((Status::Unauthorized, "Cannot find the match id in the path"))
            }
        };

        let match_id = match decode_id(match_id.to_owned()) {
            Ok(match_id) => match_id,
            Err(_) => {
                return Failure((
                    Status::InternalServerError,
                    "Cannot parse the requested match id",
                ))
            }
        };

        let factory = try_outcome!(request.guard::<&Factory>().await);

        match factory
            .get::<MatchRepository>()
            .is_allowed(match_id, user.id)
            .await
        {
            Ok(allowed) => {
                if allowed {
                    Success(request.local_cache(move || MatchUser {
                        id: user.id,
                        email: user.email.clone(),
                    }))
                } else {
                    Failure((Status::Unauthorized, "You do not have access to this match"))
                }
            }
            Err(_) => Failure((Status::Unauthorized, "Something unexpected happened")),
        }
    }
}

pub fn get_from_request(request: &Request, idk: &str) -> Result<String, Box<dyn Error>> {
    let route = request
        .route()
        .ok_or("Cannot find the requested resource")?;

    let resource = request
        .uri()
        .path()
        .segments()
        .zip(route.uri.origin.path().segments())
        .find_map(|(value, mask)| {
            if mask.contains(idk) {
                Some(value)
            } else {
                None
            }
        })
        .ok_or("Cannot find the requested resource")?;

    Ok(resource.to_owned())
}
