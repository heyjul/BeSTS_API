use rocket::serde::json::Json;

use crate::{
    models::{
        auth::{MatchUser, RoomUser},
        error::{Errors, ServerError},
        r#match::{
            CloseMatchRequestDto, CreateMatchRequest, CreateMatchRequestDto, FullMatchDto,
            MatchDto, MatchResultDto,
        },
    },
    repositories::{factory::Factory, match_repository::MatchRepository},
    utils::hasher::decode_id,
};

#[get("/<room_id>")]
pub async fn get(
    room_id: String,
    factory: &Factory,
    user: &RoomUser,
) -> ServerError<Json<Vec<FullMatchDto>>> {
    let room_id = decode_id(room_id)?;

    let matches = factory
        .get::<MatchRepository>()
        .get(room_id, user.id)
        .await?
        .into_iter()
        .map(FullMatchDto::from)
        .collect();

    Ok(Json(matches))
}

#[get("/match/<match_id>")]
pub async fn get_by_id(
    match_id: String,
    factory: &Factory,
    user: &MatchUser,
) -> ServerError<Json<FullMatchDto>> {
    let match_id = decode_id(match_id)?;

    let r#match = factory
        .get::<MatchRepository>()
        .get_by_id(match_id, user.id)
        .await?
        .ok_or(Errors::NotFound("Match not found"))?;

    Ok(Json(r#match.into()))
}

#[put("/<_room_id>", data = "<req>")]
pub async fn create_or_update(
    _room_id: String,
    req: Json<CreateMatchRequestDto>,
    factory: &Factory,
    _user: &RoomUser,
) -> ServerError<Json<MatchDto>> {
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

#[delete("/<match_id>")]
pub async fn delete(match_id: String, factory: &Factory, user: &MatchUser) -> ServerError<()> {
    let match_id = decode_id(match_id)?;

    factory
        .get::<MatchRepository>()
        .delete(match_id, user.id)
        .await?;

    Ok(())
}

#[post("/<match_id>/close", data = "<req>")]
pub async fn close(
    match_id: String,
    req: Json<CloseMatchRequestDto>,
    factory: &Factory,
    _user: &MatchUser,
) -> ServerError<Json<MatchResultDto>> {
    let match_id = decode_id(match_id)?;

    let result = factory
        .get::<MatchRepository>()
        .close(match_id, req.into_inner().into())
        .await?;

    Ok(Json(result.into()))
}
