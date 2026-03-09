use std::ops::Deref;

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Query(String);

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum QueryError {
    #[error("query is empty")]
    Empty,
    #[error("query is too long")]
    TooLong,
}

impl TryFrom<&str> for Query {
    type Error = QueryError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl Deref for Query {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Query {
    pub fn new(query: &str) -> Result<Self, QueryError> {
        match query.len() {
            0 => Err(QueryError::Empty),
            50.. => Err(QueryError::TooLong),
            _ => Ok(Self(query.into())),
        }
    }

    pub fn new_unchecked(query: &str) -> Self {
        Self(query.into())
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}
