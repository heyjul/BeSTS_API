use thiserror::Error;

#[derive(Error, Debug, Responder)]
pub enum MatchError {
    #[error("You do not have access to these matches.")]
    #[response(status = 500)]
    Forbidden(()),
    #[error("Something whent wrong.")]
    #[response(status = 500)]
    InternalServerError(()),
}

impl From<Box<dyn std::error::Error>> for MatchError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        if let Ok(err) = error.downcast::<MatchError>() {
            return *err;
        }

        MatchError::InternalServerError(())
    }
}
