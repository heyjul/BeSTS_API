use thiserror::Error;

#[derive(Error, Debug, Responder)]
pub enum AuthError {
    #[error("Email is already taken.")]
    #[response(status = 400)]
    EmailTaken(()),
    #[error("Username is already taken.")]
    #[response(status = 400)]
    UsernameTaken(()),
    #[error("Invalid credentials.")]
    #[response(status = 400)]
    InvalidCredentials(()),
    #[error("Something whent wrong.")]
    #[response(status = 500)]
    InternalServerError(()),
}

impl From<Box<dyn std::error::Error>> for AuthError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        if let Ok(err) = error.downcast::<AuthError>() {
            return *err;
        }

        AuthError::InternalServerError(())
    }
}
