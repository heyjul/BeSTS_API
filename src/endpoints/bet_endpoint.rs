use rocket::serde::json::Json;

use crate::{
    models::{
        auth::MatchUser,
        bet::{BetDto, CreateBetRequest, CreateBetRequestDto},
        error::ServerError,
    },
    repositories::{bet_repository::BetRepository, factory::Factory},
    utils::hasher::decode_id,
};

#[put("/<match_id>", data = "<req>")]
pub async fn create_or_update(
    match_id: String,
    req: Json<CreateBetRequestDto>,
    factory: &Factory,
    user: &MatchUser,
) -> ServerError<Json<BetDto>> {
    let req: CreateBetRequest = req.into_inner().into();
    let match_id = decode_id(match_id)?;

    let repo = factory.get::<BetRepository>();
    let bet = if repo.get_by_id(match_id, user.id).await?.is_some() {
        repo.update(req, match_id, user.id).await?
    } else {
        repo.create(req, match_id, user.id).await?
    };

    Ok(Json(bet.into()))
}
