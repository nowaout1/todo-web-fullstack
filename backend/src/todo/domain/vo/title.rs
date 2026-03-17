use std::ops::Deref;

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Title(String);

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TitleError {
    #[error("title is empty")]
    Empty,
    #[error("title is too short")]
    TooShort,
    #[error("title is too long")]
    TooLong,
}

impl TryFrom<&str> for Title {
    type Error = TitleError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl Deref for Title {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Title {
    pub fn new(title: &str) -> Result<Self, TitleError> {
        match title.len() {
            0 => Err(TitleError::Empty),
            ..3 => Err(TitleError::TooShort),
            50.. => Err(TitleError::TooLong),
            _ => Ok(Self(title.into())),
        }
    }

    pub fn new_unchecked(title: &str) -> Self {
        Self(title.into())
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}
