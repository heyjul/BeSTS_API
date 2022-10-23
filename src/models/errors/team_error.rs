use thiserror::Error;

#[derive(Error, Debug, Responder)]
pub enum TeamError {
    #[error("Something whent wrong.")]
    #[response(status = 500)]
    InternalServerError(()),
}

impl From<Box<dyn std::error::Error>> for TeamError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        if let Ok(err) = error.downcast::<TeamError>() {
            return *err;
        }

        TeamError::InternalServerError(())
    }
}
