use std::num::ParseIntError;

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(usize);

#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[error("failed to parse id")]
pub struct IdParseError(#[from] ParseIntError);

impl Id {
    pub fn new(id: impl Into<usize>) -> Self {
        Self(id.into())
    }

    /// Parses a string slice into an `Id`.
    ///
    /// # Panics
    /// Panics if the string cannot be parsed as a valid ID.
    pub fn from_str(s: &str) -> Self {
        Self(s.parse().expect("invalid id string"))
    }

    /// Creates an `Id` from a signed integer.
    ///
    /// # Panics
    /// Panics if the value is negative.
    pub fn from_signed_integer(id: impl Into<i64>) -> Self {
        let id = id.into();
        assert!(!id.is_negative(), "id cannot be negative");
        Self(id as usize)
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }

    pub fn into_inner(self) -> usize {
        self.0
    }
}

impl TryFrom<&str> for Id {
    type Error = IdParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse().map(Self).map_err(IdParseError)
    }
}
