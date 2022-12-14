use chrono::Local;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

static JWT_SECRET_KEY: Lazy<String> =
    Lazy::new(|| std::env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set"));

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub grant_type: String,
    pub email: String,
    pub username: String,
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    fn new(grant_type: String, exp: i64, sub: String, email: String, username: String) -> Claims {
        let now = Local::now().timestamp_millis();
        Claims {
            grant_type,
            email,
            username,
            sub,
            exp: now + exp,
            iat: now,
        }
    }
}

pub fn get_token(
    grant_type: String,
    exp: i64,
    sub: String,
    email: String,
    username: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let claims = Claims::new(grant_type, exp, sub, email, username);
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET_KEY.as_bytes()),
    )?;

    Ok(token)
}

pub fn verify(token: &str) -> Result<Claims, Box<dyn std::error::Error>> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET_KEY.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )?;

    Ok(token_data.claims)
}
