use crate::todo::{domain::TitleError, presentation::common::AppError};

impl From<TitleError> for AppError {
    fn from(value: TitleError) -> Self {
        match value {
            TitleError::Empty => AppError::text("title is empty"),
            TitleError::TooShort => AppError::text("title is too short"),
            TitleError::TooLong => AppError::text("title is too long"),
        }
    }
}
