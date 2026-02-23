use thiserror::Error;

use crate::todo::{
    entity::Todo,
    repository::{FetchLimit, FetchOffset, SearchTodosQuery, TodoRepository, TodoRepositoryError},
    vo::{
        LimitError, OffsetError, Query, QueryError, limit::DEFAULT_LIMIT, offset::DEFAULT_OFFSET,
    },
};

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SearchTodos<'a> {
    query: &'a str,
    offset: i32,
    limit: i32,
}

impl<'a> SearchTodos<'a> {
    pub fn new(query: &'a str, offset: Option<i32>, limit: Option<i32>) -> Self {
        let offset = offset.unwrap_or(DEFAULT_OFFSET as _);
        let limit = limit.unwrap_or(DEFAULT_LIMIT as _);

        Self {
            query,
            offset,
            limit,
        }
    }

    pub fn query(&self) -> &'a str {
        &self.query
    }

    pub fn offset(&self) -> i32 {
        self.offset
    }

    pub fn limit(&self) -> i32 {
        self.limit
    }
}

#[derive(Error, Debug)]
pub enum SearchTodosError {
    #[error("got invalid query: {0}")]
    Query(#[from] QueryError),
    #[error("got invalid offset: {0}")]
    Offset(#[from] OffsetError),
    #[error("got invalid limit: {0}")]
    Limit(#[from] LimitError),
    #[error("failed to fetch todo by id: {0}")]
    Repository(#[from] TodoRepositoryError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SearchTodosUseCase<'a, R>
where
    R: TodoRepository,
{
    repository: &'a R,
}

impl<'a, R> SearchTodosUseCase<'a, R>
where
    R: TodoRepository,
{
    pub fn new(repository: &'a R) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        SearchTodos {
            query,
            offset,
            limit,
        }: SearchTodos<'_>,
    ) -> Result<Vec<Todo>, SearchTodosError> {
        let query = Query::new(query).map_err(SearchTodosError::Query)?;
        let offset = FetchOffset::new(offset).map_err(SearchTodosError::Offset)?;
        let limit = FetchLimit::new(limit).map_err(SearchTodosError::Limit)?;

        let data = SearchTodosQuery::new(query, offset, limit);

        let todo = self
            .repository
            .search_todos(data)
            .await
            .map_err(SearchTodosError::Repository)?;

        Ok(todo)
    }
}
