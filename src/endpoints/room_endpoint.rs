use rocket::serde::json::Json;

use crate::{
    models::{
        auth::{RoomUser, User},
        r#match::FullMatchDto,
        room::{CreateRoomRequest, FullRoomDto, RoomDto},
        room_error::RoomError,
    },
    repositories::{
        factory::Factory, match_repository::MatchRepository, room_repository::RoomRepository,
    },
    utils::hasher::decode_id,
};

#[get("/")]
pub async fn get(factory: &Factory, user: &User) -> Result<Json<Vec<RoomDto>>, RoomError> {
    let rooms = factory
        .get::<RoomRepository>()
        .get_rooms(user.id)
        .await?
        .into_iter()
        .map(RoomDto::from)
        .collect();

    Ok(Json(rooms))
}

#[get("/<room_id>")]
pub async fn get_by_id(
    room_id: String,
    factory: &Factory,
    user: &RoomUser,
) -> Result<Json<FullRoomDto>, RoomError> {
    let id = decode_id(room_id)?;

    let room = factory
        .get::<RoomRepository>()
        .get_room(id)
        .await?
        .map(RoomDto::from)
        .ok_or(RoomError::RoomNotFound(()))?;

    let matches: Vec<_> = factory
        .get::<MatchRepository>()
        .get(id, user.id)
        .await?
        .into_iter()
        .map(FullMatchDto::from)
        .collect();

    Ok(Json(FullRoomDto::new(room, matches)))
}

#[post("/", data = "<req>")]
pub async fn create(
    req: Json<CreateRoomRequest>,
    factory: &Factory,
    user: &User,
) -> Result<Json<RoomDto>, RoomError> {
    let room = factory
        .get::<RoomRepository>()
        .create(req.into_inner(), user.id)
        .await
        .map(RoomDto::from)?;

    Ok(Json(room))
}

#[post("/join/<id>")]
pub async fn join(id: String, factory: &Factory, user: &User) -> Result<Json<RoomDto>, RoomError> {
    let id = decode_id(id)?;

    let room = factory
        .get::<RoomRepository>()
        .join(id, user.id)
        .await
        .map(RoomDto::from)?;

    Ok(Json(room))
}

#[delete("/<id>")]
pub async fn delete(id: String, factory: &Factory, user: &User) -> Result<(), RoomError> {
    let id = decode_id(id)?;

    factory.get::<RoomRepository>().delete(id, user.id).await?;

    Ok(())
}
