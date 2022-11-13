use crate::{
    models::{
        auth::{LoginRequest, RefreshTokenRequest, RegisterRequest, Token},
        error::{Errors, ServerError},
    },
    repositories::{auth_repository::AuthRepository, factory::Factory},
    utils::{hasher::encode_id, jwt, password},
};
use rocket::serde::json::Json;

#[post("/register", data = "<req>")]
pub async fn register(req: Json<RegisterRequest>, factory: &Factory) -> ServerError<()> {
    factory
        .get::<AuthRepository>()
        .register(req.into_inner())
        .await?;

    Ok(())
}

#[post("/login", data = "<req>")]
pub async fn login(req: Json<LoginRequest>, factory: &Factory) -> ServerError<Json<Token>> {
    let user = factory
        .get::<AuthRepository>()
        .get_user_by_email(&req.email)
        .await?
        .ok_or(Errors::InvalidCredentials("Wrong email and/or password"))?;

    let login_result = password::verify(&user.password, &req.password)?;

    if !login_result {
        return Err(Errors::InvalidCredentials("Wrong email and/or password"));
    }

    let response = Token {
        token: jwt::get_token(
            "normal".to_owned(),
            3600,
            encode_id(user.id),
            user.email.clone(),
            user.username.clone(),
        )?,
        refresh_token: jwt::get_token(
            "refresh".to_owned(),
            86400 * 7,
            encode_id(user.id),
            user.email,
            user.username,
        )?,
    };

    jwt::verify(&response.token)?;

    Ok(Json(response))
}

#[post("/refresh-token", data = "<req>")]
pub async fn refresh_token(req: Json<RefreshTokenRequest>) -> ServerError<Json<Token>> {
    let req = req.into_inner();

    let token_info = jwt::verify(&req.refresh_token)?;

    let response = Token {
        token: jwt::get_token(
            "normal".to_owned(),
            3600,
            token_info.sub,
            token_info.email,
            token_info.username,
        )?,
        refresh_token: req.refresh_token,
    };

    Ok(Json(response))
}
