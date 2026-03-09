use thiserror::Error;

use crate::todo::{
    application::repository::{FetchRecentTodosQuery, TodoRepository, TodoRepositoryError},
    domain::{Cursor, Pagination, entity::Todo},
};

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GetRecentTodos {
    pagination: Pagination,
}

impl GetRecentTodos {
    pub fn new(pagination: Pagination) -> Self {
        Self { pagination }
    }
}

#[derive(Error, Debug)]
pub enum GetRecentTodosError {
    #[error("failed to fetch todo by id: {0}")]
    Repository(#[from] TodoRepositoryError),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetRecentTodosResponse {
    todos: Vec<Todo>,
    next_cursor: Option<Cursor>,
}

impl GetRecentTodosResponse {
    pub fn new(todos: Vec<Todo>, next_cursor: Option<Cursor>) -> Self {
        Self { todos, next_cursor }
    }

    pub fn todos(&self) -> &[Todo] {
        &self.todos[..]
    }

    pub fn next_cursor(&self) -> Option<&Cursor> {
        self.next_cursor.as_ref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetRecentTodosUseCase<'a, R>
where
    R: TodoRepository,
{
    repository: &'a R,
}

impl<'a, R> GetRecentTodosUseCase<'a, R>
where
    R: TodoRepository,
{
    pub fn new(repository: &'a R) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        GetRecentTodos { pagination }: GetRecentTodos,
    ) -> Result<GetRecentTodosResponse, GetRecentTodosError> {
        let query = FetchRecentTodosQuery::new(pagination);

        let result = self
            .repository
            .fetch_recent_todos(query)
            .await
            .map_err(GetRecentTodosError::Repository)?;

        let response = GetRecentTodosResponse {
            todos: result.todos().into(),
            next_cursor: result.next_cursor().cloned(),
        };

        Ok(response)
    }
}
