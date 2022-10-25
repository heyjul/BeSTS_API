use rocket::serde::json::Json;

use crate::{
    models::{
        auth::RoomUser,
        match_error::MatchError,
        r#match::{CreateMatchRequest, CreateMatchRequestDto, MatchDto},
    },
    repositories::{factory::Factory, match_repository::MatchRepository},
    utils::hasher::decode_id,
};

#[get("/<room_id>")]
pub async fn get(
    room_id: String,
    factory: &Factory,
    user: &RoomUser,
) -> Result<Json<Vec<MatchDto>>, MatchError> {
    let room_id = decode_id(room_id)?;

    if !user.rooms.iter().any(|&r| r == room_id) {
        return Err(MatchError::Forbidden(()));
    }

    let matches = factory
        .get::<MatchRepository>()
        .get(room_id)
        .await?
        .into_iter()
        .map(MatchDto::from)
        .collect();

    Ok(Json(matches))
}

#[put("/<room_id>", data = "<req>")]
pub async fn create_or_update(
    room_id: String,
    req: Json<CreateMatchRequestDto>,
    factory: &Factory,
    user: &RoomUser,
) -> Result<Json<MatchDto>, MatchError> {
    let room_id = decode_id(room_id)?;

    if !user.rooms.iter().any(|&r| r == room_id) {
        return Err(MatchError::Forbidden(()));
    }

    let req: CreateMatchRequest = req.into_inner().try_into()?;

    let r#match = if req.id.is_some() {
        factory
            .get::<MatchRepository>()
            .update(req)
            .await
            .map(MatchDto::from)?
    } else {
        factory
            .get::<MatchRepository>()
            .create(req)
            .await
            .map(MatchDto::from)?
    };

    Ok(Json(r#match))
}
