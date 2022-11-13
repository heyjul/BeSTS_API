use rocket::serde::json::Json;

use crate::{
    models::{error::ServerError, team::TeamDto},
    repositories::{factory::Factory, team_repository::TeamRepository},
};

#[get("/")]
pub async fn get(factory: &Factory) -> ServerError<Json<Vec<TeamDto>>> {
    let teams = factory
        .get::<TeamRepository>()
        .get()
        .await?
        .into_iter()
        .map(TeamDto::from)
        .collect();

    Ok(Json(teams))
}
