use rocket::{
    http::Status,
    request::{self, FromRequest},
    Request,
};
use serde::{Deserialize, Serialize};

use crate::utils::jwt;

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

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        if let Some(bearer) = request.headers().get_one("Authorization") {
            if let Some((_, token)) = bearer.split_once(' ') {
                let claims = match jwt::verify(token) {
                    Ok(claims) => claims,
                    Err(_) => {
                        return request::Outcome::Failure((
                            Status::Unauthorized,
                            "Cannot parse jwt token",
                        ))
                    }
                };

                let user = User {
                    id: claims.sub,
                    email: claims.email,
                };

                return request::Outcome::Success(request.local_cache(move || user));
            }
        }

        request::Outcome::Failure((Status::Unauthorized, "You must be connected"))
    }
}
