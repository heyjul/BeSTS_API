use rocket::{
    http::Status,
    outcome::{try_outcome, Outcome::*},
    request::{FromRequest, Outcome},
    Request,
};
use serde::{Deserialize, Serialize};

use crate::{
    repositories::{factory::Factory, room_repository::RoomRepository},
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

        let room_id = if let Some(route) = request.route() {
            request
                .uri()
                .path()
                .segments()
                .zip(route.uri.origin.path().segments())
                .find_map(|(value, mask)| {
                    if mask.contains("room_id") {
                        Some(value)
                    } else {
                        None
                    }
                })
        } else {
            return Failure((Status::Unauthorized, "Cannot find the requested room id"));
        };

        let room_id = match room_id {
            Some(room_id) => room_id,
            None => return Failure((Status::Unauthorized, "Cannot find the requested room id")),
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
