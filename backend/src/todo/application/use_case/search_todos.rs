use thiserror::Error;

use crate::todo::{
    application::repository::{SearchTodosQuery, TodoRepository, TodoRepositoryError},
    domain::{
        Cursor,
        entity::Todo,
        vo::{Pagination, Query, Sort},
    },
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SearchTodos {
    query: Query,
    pagination: Pagination,
    sort: Sort,
}

impl SearchTodos {
    pub fn new(query: Query, pagination: Pagination, sort: Sort) -> Self {
        Self {
            query,
            pagination,
            sort,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SearchTodosResponse {
    todos: Vec<Todo>,
    next_cursor: Option<Cursor>,
}

impl SearchTodosResponse {
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

#[derive(Error, Debug)]
pub enum SearchTodosError {
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
            pagination,
            sort,
        }: SearchTodos,
    ) -> Result<SearchTodosResponse, SearchTodosError> {
        let data = SearchTodosQuery::new(query, pagination, sort);

        let result = self
            .repository
            .search_todos(data)
            .await
            .map_err(SearchTodosError::Repository)?;

        let response = SearchTodosResponse {
            todos: result.todos().into(),
            next_cursor: result.next_cursor().cloned(),
        };

        Ok(response)
    }
}
