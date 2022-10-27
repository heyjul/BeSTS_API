use thiserror::Error;

#[derive(Error, Debug, Responder)]
pub enum BetError {
    #[error("Something whent wrong.")]
    #[response(status = 500)]
    InternalServerError(()),
}

impl From<Box<dyn std::error::Error>> for BetError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        if let Ok(err) = error.downcast::<BetError>() {
            return *err;
        }

        BetError::InternalServerError(())
    }
}
