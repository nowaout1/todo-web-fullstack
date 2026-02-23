use thiserror::Error;

pub const DEFAULT_OFFSET: usize = 0;

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OffsetError {
    #[error("offset must be more or equal than {0} value")]
    TooSmall(usize),
    #[error("offset must be less or equal than {0} value")]
    TooMuch(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Offset<const MIN: usize, const MAX: usize>(usize);

impl<const MIN: usize, const MAX: usize> Default for Offset<MIN, MAX> {
    fn default() -> Self {
        Self(DEFAULT_OFFSET)
    }
}

impl<const MIN: usize, const MAX: usize> Offset<MIN, MAX> {
    pub fn new(offset: impl Into<i64>) -> Result<Self, OffsetError> {
        match offset.into() as _ {
            x if x < MIN => Err(OffsetError::TooSmall(MIN)),
            x if x > MAX => Err(OffsetError::TooMuch(MAX)),
            x => Ok(Self(x)),
        }
    }

    pub fn into_inner(self) -> usize {
        self.0
    }
}
