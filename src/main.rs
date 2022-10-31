#[macro_use]
extern crate rocket;

use endpoints::*;
use repositories::factory::SoccerDb;
use rocket::{
    fairing::{self, AdHoc, Fairing, Info, Kind},
    http::Header,
    Build, Request, Response, Rocket,
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
        .attach(CORS)
        .attach(SoccerDb::init())
        .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
        .mount(
            "/",
            routes![
                auth_endpoint::register,
                auth_endpoint::login,
                auth_endpoint::refresh_token,
                all_options,
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
            routes![
                match_endpoint::get,
                match_endpoint::create_or_update,
                match_endpoint::delete
            ],
        )
        .mount("/bets", routes![bet_endpoint::create_or_update])
}

/// Catches all OPTION requests in order to get the CORS related Fairing triggered.
#[options("/<_..>")]
fn all_options() {
    /* Intentionally left empty */
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, DELETE, OPTIONS, PUT",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
