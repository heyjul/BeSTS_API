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
    _user: &RoomUser,
) -> Result<Json<Vec<MatchDto>>, MatchError> {
    let room_id = decode_id(room_id)?;

    let matches = factory
        .get::<MatchRepository>()
        .get(room_id)
        .await?
        .into_iter()
        .map(MatchDto::from)
        .collect();

    Ok(Json(matches))
}

#[put("/<_room_id>", data = "<req>")]
pub async fn create_or_update(
    _room_id: String,
    req: Json<CreateMatchRequestDto>,
    factory: &Factory,
    _user: &RoomUser,
) -> Result<Json<MatchDto>, MatchError> {
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
