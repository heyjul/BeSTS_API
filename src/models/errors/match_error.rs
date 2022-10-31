use thiserror::Error;

#[derive(Error, Debug, Responder)]
pub enum MatchError {
    #[error("Something whent wrong.")]
    #[response(status = 500)]
    InternalServerError(()),
    #[error("Not found.")]
    #[response(status = 404)]
    NotFound(()),
}

impl From<Box<dyn std::error::Error>> for MatchError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        if let Ok(err) = error.downcast::<MatchError>() {
            return *err;
        }

        MatchError::InternalServerError(())
    }
}
