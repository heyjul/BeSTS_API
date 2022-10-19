use crate::{
    models::{
        auth::{LoginRequest, RefreshTokenRequest, RegisterRequest, Token},
        auth_error::AuthError,
    },
    repositories::{auth_repository::AuthRepository, factory::Factory},
    utils::{jwt, password},
};
use rocket::serde::json::Json;

#[post("/register", data = "<req>")]
pub async fn register(
    req: Json<RegisterRequest>,
    factory: &Factory,
) -> Result<Json<bool>, AuthError> {
    let registered = factory
        .get::<AuthRepository>()
        .register(req.into_inner())
        .await
        .map_err(AuthError::from)?;

    Ok(Json(registered))
}

#[post("/login", data = "<req>")]
pub async fn login(req: Json<LoginRequest>, factory: &Factory) -> Result<Json<Token>, AuthError> {
    let user = factory
        .get::<AuthRepository>()
        .get_user_by_email(&req.email)
        .await
        .map_err(AuthError::from)?
        .ok_or_else(|| AuthError::InvalidCredentials("TODO".to_owned()))?;

    let login_result = password::verify(&user.password, &req.password).map_err(AuthError::from)?;

    if !login_result {
        return Err(AuthError::InvalidCredentials("".to_owned()));
    }

    let response = Token {
        token: jwt::get_token(
            "normal".to_owned(),
            3600,
            user.id,
            user.email.clone(),
            user.username.clone(),
        )
        .map_err(AuthError::from)?,
        refresh_token: jwt::get_token(
            "refresh".to_owned(),
            86400 * 7,
            user.id,
            user.email,
            user.username,
        )
        .map_err(AuthError::from)?,
    };

    jwt::verify(&response.token).map_err(AuthError::from)?;

    Ok(Json(response))
}

#[post("/refresh-token", data = "<req>")]
pub async fn refresh_token(req: Json<RefreshTokenRequest>) -> Result<Json<Token>, AuthError> {
    let req = req.into_inner();

    let token_info = jwt::verify(&req.refresh_token).map_err(AuthError::from)?;

    let response = Token {
        token: jwt::get_token(
            "normal".to_owned(),
            3600,
            token_info.sub,
            token_info.email,
            token_info.username,
        )
        .map_err(AuthError::from)?,
        refresh_token: req.refresh_token,
    };

    Ok(Json(response))
}
