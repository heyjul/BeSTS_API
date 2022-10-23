use rocket::serde::json::Json;

use crate::{
    models::{team::TeamDto, team_error::TeamError},
    repositories::{factory::Factory, team_repository::TeamRepository},
    utils::hasher::encode_id,
};

#[get("/")]
pub async fn get(factory: &Factory) -> Result<Json<Vec<TeamDto>>, TeamError> {
    let teams = factory
        .get::<TeamRepository>()
        .get()
        .await?
        .into_iter()
        .map(|t| TeamDto {
            id: encode_id(t.id),
            name: t.name,
        })
        .collect();

    Ok(Json(teams))
}
