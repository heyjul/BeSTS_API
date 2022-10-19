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

async fn load_config(rocket: Rocket<Build>) -> Rocket<Build> {
    dotenvy::dotenv().ok();
    rocket
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(AdHoc::on_ignite("Load config", load_config))
        .attach(SoccerDb::init())
        .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
        .mount(
            "/",
            routes![
                auth_endpoint::register,
                auth_endpoint::login,
                auth_endpoint::refresh_token
            ],
        )
}
