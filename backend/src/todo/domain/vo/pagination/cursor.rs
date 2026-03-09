use thiserror::Error;

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CursorError {
    #[error("cursor cannot be zero")]
    Zero,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cursor(usize);

impl Cursor {
    pub fn new(cursor: impl Into<usize>) -> Result<Self, CursorError> {
        let cursor = cursor.into();

        if cursor == 0 {
            return Err(CursorError::Zero);
        }

        Ok(Self(cursor))
    }

    pub fn new_unchecked(cursor: impl Into<usize>) -> Self {
        Self(cursor.into())
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }

    pub fn into_inner(self) -> usize {
        self.0
    }
}
