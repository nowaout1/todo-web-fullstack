use crate::todo::{domain::IdParseError, presentation::common::AppError};

impl From<IdParseError> for AppError {
    fn from(_value: IdParseError) -> Self {
        AppError::text("got invalid identifier")
    }
}
