use once_cell::sync::Lazy;
use rocket::serde::json::Json;

static HASHER: Lazy<hashids::HashIds> = Lazy::new(|| {
    hashids::HashIds::new_with_salt("Sel de Guérande".to_owned()).expect("Could not create hashids")
});

use crate::{
    models::{
        auth::User,
        room::{CreateRoomRequest, Room, RoomWithUrl},
        room_error::RoomError,
    },
    repositories::{factory::Factory, room_repository::RoomRepository},
};

#[get("/")]
pub async fn get(factory: &Factory, user: &User) -> Result<Json<Vec<Room>>, RoomError> {
    let rooms = factory.get::<RoomRepository>().get_rooms(user.id).await?;

    Ok(Json(rooms))
}

#[get("/<url>")]
pub async fn get_by_id(
    url: String,
    factory: &Factory,
    user: &User,
) -> Result<Json<RoomWithUrl>, RoomError> {
    let id = match HASHER.decode(url).first() {
        Some(&id) => id,
        None => return Err(RoomError::InvalidArgumentError(())),
    };

    let room = factory
        .get::<RoomRepository>()
        .get_room(id)
        .await?
        .map(|r| {
            let mut room = RoomWithUrl {
                name: r.name.clone(),
                url: None,
            };

            if r.owner == user.id {
                room.url = Some(HASHER.encode(&vec![r.id]))
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
) -> Result<Json<RoomWithUrl>, RoomError> {
    let room = factory
        .get::<RoomRepository>()
        .create(req.into_inner(), user.id)
        .await
        .map(|r| RoomWithUrl {
            name: r.name,
            url: Some(HASHER.encode(&vec![r.id])),
        })?;

    Ok(Json(room))
}

#[post("/join/<url>")]
pub async fn join(url: String, factory: &Factory, user: &User) -> Result<Json<Room>, RoomError> {
    let id = match HASHER.decode(url).first() {
        Some(&id) => id,
        None => return Err(RoomError::InvalidArgumentError(())),
    };

    let room = factory.get::<RoomRepository>().join(id, user.id).await?;

    Ok(Json(room))
}

#[delete("/<url>")]
pub async fn delete(url: String, factory: &Factory, user: &User) -> Result<(), RoomError> {
    let id = match HASHER.decode(url).first() {
        Some(&id) => id,
        None => return Err(RoomError::InvalidArgumentError(())),
    };

    factory.get::<RoomRepository>().delete(id, user.id).await?;

    Ok(())
}
