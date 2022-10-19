use rocket::{
    http::Status,
    request::{self, FromRequest},
    Request,
};
use rocket_db_pools::Database;
use sqlx::{Pool, Sqlite, SqlitePool};

#[derive(Database)]
#[database("soccer")]
pub struct SoccerDb(pub rocket_db_pools::sqlx::SqlitePool);

pub struct Factory(pub SqlitePool);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for &'r Factory {
    type Error = &'r str;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        if let Some(db) = SoccerDb::fetch(request.rocket()) {
            let pool = db.0.clone();
            return request::Outcome::Success(request.local_cache(move || Factory(pool)));
        }

        request::Outcome::Failure((Status::InternalServerError, "Internal server error"))
    }
}

pub trait RepositoryFactory {
    fn new(db_pool: Pool<Sqlite>) -> Self;
}

impl Factory {
    pub fn get<T>(&self) -> T
    where
        T: RepositoryFactory,
    {
        T::new(self.0.clone())
    }
}
