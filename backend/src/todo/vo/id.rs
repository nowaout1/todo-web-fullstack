use std::str::FromStr;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id(Uuid);

impl Default for Id {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Error, Debug, Clone, PartialEq, Eq, Hash)]
#[error("failed to parse id: {0}")]
pub struct IdParseError(#[from] uuid::Error);

impl Id {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn new_unchecked(id: &str) -> Self {
        Self(Uuid::try_parse(id).expect("got invalid uuid"))
    }

    pub fn into_inner(self) -> Uuid {
        self.0
    }
}

impl TryFrom<&str> for Id {
    type Error = IdParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Uuid::from_str(value).map(Self).map_err(IdParseError)
    }
}
