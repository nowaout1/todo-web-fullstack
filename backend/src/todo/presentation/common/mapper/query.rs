use crate::todo::{domain::QueryError, presentation::common::AppError};

impl From<QueryError> for AppError {
    fn from(value: QueryError) -> Self {
        match value {
            QueryError::Empty => AppError::text("query is empty"),
            QueryError::TooLong => AppError::text("query is too long"),
        }
    }
}
