use thiserror::Error;

pub const DEFAULT_LIMIT: usize = 32;

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LimitError {
    #[error("limit must be more or equal than {0} value")]
    TooSmall(usize),
    #[error("limit must be less or equal than {0} value")]
    TooMuch(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Limit<const MIN: usize, const MAX: usize>(usize);

impl<const MIN: usize, const MAX: usize> Default for Limit<MIN, MAX> {
    fn default() -> Self {
        Self(DEFAULT_LIMIT)
    }
}

impl<const MIN: usize, const MAX: usize> Limit<MIN, MAX> {
    pub fn new(limit: impl Into<i64>) -> Result<Self, LimitError> {
        match limit.into() as _ {
            x if x < MIN => Err(LimitError::TooSmall(MIN)),
            x if x > MAX => Err(LimitError::TooMuch(MAX)),
            x => Ok(Self(x)),
        }
    }

    pub fn into_inner(self) -> usize {
        self.0
    }
}
