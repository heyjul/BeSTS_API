use rocket::serde::json::Json;

use crate::{
    models::{
        auth::{RoomUser, User},
        error::{Errors, ServerError},
        room::{CreateRoomRequest, RoomDto},
    },
    repositories::{factory::Factory, room_repository::RoomRepository},
    utils::hasher::decode_id,
};

#[get("/")]
pub async fn get(factory: &Factory, user: &User) -> ServerError<Json<Vec<RoomDto>>> {
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
    _user: &RoomUser,
) -> ServerError<Json<RoomDto>> {
    let id = decode_id(room_id)?;

    let room = factory
        .get::<RoomRepository>()
        .get_room(id)
        .await?
        .map(RoomDto::from)
        .ok_or(Errors::NotFound("Room not found."))?;

    Ok(Json(room))
}

#[post("/", data = "<req>")]
pub async fn create(
    req: Json<CreateRoomRequest>,
    factory: &Factory,
    user: &User,
) -> ServerError<Json<RoomDto>> {
    let room = factory
        .get::<RoomRepository>()
        .create(req.into_inner(), user.id)
        .await
        .map(RoomDto::from)?;

    Ok(Json(room))
}

#[post("/join/<id>")]
pub async fn join(id: String, factory: &Factory, user: &User) -> ServerError<Json<RoomDto>> {
    let id = decode_id(id)?;

    let room = factory
        .get::<RoomRepository>()
        .join(id, user.id)
        .await
        .map(RoomDto::from)?;

    Ok(Json(room))
}

#[delete("/<id>")]
pub async fn delete(id: String, factory: &Factory, user: &User) -> ServerError<()> {
    let id = decode_id(id)?;

    factory.get::<RoomRepository>().delete(id, user.id).await?;

    Ok(())
}
