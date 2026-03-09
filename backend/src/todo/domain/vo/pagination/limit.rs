use thiserror::Error;

pub type Limit = ClampedLimit<1, 100>;

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LimitError {
    #[error("limit {actual} is less than minimum {min}")]
    TooSmall { min: usize, actual: usize },
    #[error("limit {actual} is greater than maximum {max}")]
    TooMuch { max: usize, actual: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClampedLimit<const MIN: usize, const MAX: usize>(usize);

impl<const MIN: usize, const MAX: usize> ClampedLimit<MIN, MAX> {
    pub fn new(limit: impl Into<usize>) -> Result<Self, LimitError> {
        match limit.into() {
            x if x < MIN => Err(LimitError::TooSmall {
                min: MIN,
                actual: x,
            }),
            x if x > MAX => Err(LimitError::TooMuch {
                max: MAX,
                actual: x,
            }),
            x => Ok(Self(x)),
        }
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }

    pub fn into_inner(self) -> usize {
        self.0
    }
}
