use crate::todo::{
    domain::{Cursor, CursorError, Limit, LimitError, Pagination},
    presentation::common::{AppError, PaginationDto},
};

impl TryFrom<PaginationDto> for Pagination {
    type Error = AppError;

    fn try_from(value: PaginationDto) -> Result<Self, Self::Error> {
        let cursor = value.cursor().map(Cursor::new).transpose()?;
        let limit = value.limit().map(Limit::new).transpose()?;

        let pagination = Pagination::new(cursor, limit);

        Ok(pagination)
    }
}

impl From<CursorError> for AppError {
    fn from(value: CursorError) -> Self {
        match value {
            CursorError::Zero => AppError::text("cursor cannot be zero"),
        }
    }
}

impl From<LimitError> for AppError {
    fn from(value: LimitError) -> Self {
        match value {
            LimitError::TooSmall { min, actual } => {
                AppError::text(&format!("limit {actual} is less than minumum {min}"))
            }
            LimitError::TooMuch { max, actual } => {
                AppError::text(&format!("limit {actual} is greater than maximum {max}"))
            }
        }
    }
}
