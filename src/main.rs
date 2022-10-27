#[macro_use]
extern crate rocket;

use endpoints::*;
use repositories::factory::SoccerDb;
use rocket::{
    fairing::{self, AdHoc},
    Build, Rocket,
};
use rocket_db_pools::Database;

mod endpoints;
mod models;
mod repositories;
mod utils;

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match SoccerDb::fetch(&rocket) {
        Some(db) => match sqlx::migrate!().run(&**db).await {
            Ok(_) => Ok(rocket),
            Err(e) => {
                error!("Failed to initialize SQLx database: {}", e);
                Err(rocket)
            }
        },
        None => Err(rocket),
    }
}

#[launch]
fn rocket() -> _ {
    dotenvy::dotenv().ok();

    rocket::build()
        .attach(SoccerDb::init())
        .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
        .mount(
            "/",
            routes![
                auth_endpoint::register,
                auth_endpoint::login,
                auth_endpoint::refresh_token,
            ],
        )
        .mount(
            "/rooms",
            routes![
                room_endpoint::get,
                room_endpoint::create,
                room_endpoint::get_by_id,
                room_endpoint::join,
                room_endpoint::delete,
            ],
        )
        .mount("/teams", routes![team_endpoints::get,])
        .mount(
            "/matches",
            routes![match_endpoint::get, match_endpoint::create_or_update],
        )
        .mount("/bets", routes![bet_endpoint::create_or_update])
}
