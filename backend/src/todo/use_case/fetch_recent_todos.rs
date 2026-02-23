use thiserror::Error;

use crate::todo::{
    entity::Todo,
    repository::{
        FetchLimit, FetchOffset, FetchRecentTodosQuery, TodoRepository, TodoRepositoryError,
    },
    vo::{LimitError, OffsetError, limit::DEFAULT_LIMIT},
};

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FetchRecentTodos {
    offset: i32,
    limit: i32,
}

impl FetchRecentTodos {
    pub fn new(offset: i32, limit: Option<i32>) -> Self {
        let limit = limit.unwrap_or(DEFAULT_LIMIT as _);

        Self { offset, limit }
    }

    pub fn offset(&self) -> i32 {
        self.offset
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }
}

#[derive(Error, Debug)]
pub enum FetchRecentTodosError {
    #[error("got invalid offset: {0}")]
    Offset(#[from] OffsetError),
    #[error("got invalid limit: {0}")]
    Limit(#[from] LimitError),
    #[error("failed to fetch todo by id: {0}")]
    Repository(#[from] TodoRepositoryError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FetchRecentTodosUseCase<'a, R>
where
    R: TodoRepository,
{
    repository: &'a R,
}

impl<'a, R> FetchRecentTodosUseCase<'a, R>
where
    R: TodoRepository,
{
    pub fn new(repository: &'a R) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        FetchRecentTodos { offset, limit }: FetchRecentTodos,
    ) -> Result<Vec<Todo>, FetchRecentTodosError> {
        let offset = FetchOffset::new(offset).map_err(FetchRecentTodosError::Offset)?;
        let limit = FetchLimit::new(limit).map_err(FetchRecentTodosError::Limit)?;

        let query = FetchRecentTodosQuery::new(offset, limit);

        let todos = self
            .repository
            .fetch_recent_todos(query)
            .await
            .map_err(FetchRecentTodosError::Repository)?;

        Ok(todos)
    }
}
