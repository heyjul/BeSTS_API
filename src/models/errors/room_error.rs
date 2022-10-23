use thiserror::Error;

#[derive(Error, Debug, Responder)]
pub enum RoomError {
    #[error("Something whent wrong.")]
    #[response(status = 500)]
    InternalServerError(()),
    #[error("Room not found.")]
    #[response(status = 404)]
    RoomNotFound(()),
    #[error("You have already joined this room.")]
    #[response(status = 400)]
    AlreadyJoined(()),
    #[error("You must be the owner to perform this operation.")]
    #[response(status = 403)]
    NotAllowed(()),
}

impl From<Box<dyn std::error::Error>> for RoomError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        if let Ok(err) = error.downcast::<RoomError>() {
            return *err;
        }

        RoomError::InternalServerError(())
    }
}
