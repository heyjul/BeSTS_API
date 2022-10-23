use rocket::serde::json::Json;

use crate::{
    models::{
        auth::User,
        room::{CreateRoomRequest, RoomDto},
        room_error::RoomError,
    },
    repositories::{factory::Factory, room_repository::RoomRepository},
    utils::hasher::{decode_id, encode_id},
};

#[get("/")]
pub async fn get(factory: &Factory, user: &User) -> Result<Json<Vec<RoomDto>>, RoomError> {
    let rooms = factory
        .get::<RoomRepository>()
        .get_rooms(user.id)
        .await?
        .into_iter()
        .map(|r| RoomDto {
            id: Some(encode_id(r.id)),
            name: r.name,
        })
        .collect();

    Ok(Json(rooms))
}

#[get("/<id>")]
pub async fn get_by_id(
    id: String,
    factory: &Factory,
    user: &User,
) -> Result<Json<RoomDto>, RoomError> {
    let id = decode_id(id)?;

    let room = factory
        .get::<RoomRepository>()
        .get_room(id)
        .await?
        .map(|r| {
            let mut room = RoomDto {
                id: None,
                name: r.name,
            };

            if r.owner_id == user.id {
                room.id = Some(encode_id(r.id))
            }

            room
        });

    match room {
        Some(room) => Ok(Json(room)),
        None => Err(RoomError::RoomNotFound(())),
    }
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
        .map(|r| RoomDto {
            id: Some(encode_id(r.id)),
            name: r.name,
        })?;

    Ok(Json(room))
}

#[post("/join/<id>")]
pub async fn join(id: String, factory: &Factory, user: &User) -> Result<Json<RoomDto>, RoomError> {
    let id = decode_id(id)?;

    let room = factory.get::<RoomRepository>().join(id, user.id).await?;

    Ok(Json(RoomDto {
        id: Some(encode_id(room.id)),
        name: room.name,
    }))
}

#[delete("/<id>")]
pub async fn delete(id: String, factory: &Factory, user: &User) -> Result<(), RoomError> {
    let id = decode_id(id)?;

    factory.get::<RoomRepository>().delete(id, user.id).await?;

    Ok(())
}
