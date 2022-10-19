use thiserror::Error;

#[derive(Error, Debug, Responder)]
pub enum AuthError {
    #[error("The email {0} is already taken.")]
    #[response(status = 400)]
    EmailTaken(String),
    #[error("The username {0} is already taken.")]
    #[response(status = 400)]
    UsernameTaken(String),
    #[error("Something whent wrong.")]
    #[response(status = 500)]
    InternalServerError(String),
    #[error("Invalid credentials.")]
    #[response(status = 400)]
    InvalidCredentials(String),
}

impl From<Box<dyn std::error::Error>> for AuthError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        if let Ok(err) = error.downcast::<AuthError>() {
            return *err;
        }

        AuthError::InternalServerError("Something whent wrong".to_owned())
    }
}
