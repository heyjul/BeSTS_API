use std::fmt::Display;

use rocket::response::Responder;

pub type Error<T> = Result<T, Box<dyn std::error::Error>>;
pub type ServerError<T> = Result<T, Errors>;

#[derive(Debug, Responder)]
pub enum Errors {
    #[response(status = 400, content_type = "json")]
    UsernameTaken(&'static str),
    #[response(status = 400)]
    EmailTaken(&'static str),
    #[response(status = 400)]
    AlreadyJoined(&'static str),
    #[response(status = 400, content_type = "json")]
    InvalidCredentials(&'static str),
    #[response(status = 403)]
    NotAllowed(&'static str),
    #[response(status = 404)]
    NotFound(&'static str),
    #[response(status = 500)]
    InternalServerError(&'static str),
}

impl Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Errors::UsernameTaken(msg)
            | Errors::EmailTaken(msg)
            | Errors::AlreadyJoined(msg)
            | Errors::InvalidCredentials(msg)
            | Errors::NotAllowed(msg)
            | Errors::NotFound(msg)
            | Errors::InternalServerError(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for Errors {}

impl From<Box<dyn std::error::Error>> for Errors {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        if let Ok(err) = error.downcast::<Errors>() {
            return *err;
        }

        Errors::InternalServerError("An error occured.")
    }
}
