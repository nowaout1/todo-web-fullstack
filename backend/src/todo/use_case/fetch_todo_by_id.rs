use thiserror::Error;

use crate::todo::{
    entity::Todo,
    repository::{FetchTodoByIdQuery, TodoRepository, TodoRepositoryError},
    vo::{Id, IdParseError},
};

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FetchTodoById<'a> {
    id: &'a str,
}

impl<'a> FetchTodoById<'a> {
    pub fn new(id: &'a str) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &'a str {
        self.id
    }
}

#[derive(Error, Debug)]
pub enum FetchTodoByIdError {
    #[error("got invalid todo id: {0}")]
    InvalidId(#[from] IdParseError),
    #[error("failed to fetch todo by id: {0}")]
    Repository(#[from] TodoRepositoryError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FetchTodoByIdUseCase<'a, R>
where
    R: TodoRepository,
{
    repository: &'a R,
}

impl<'a, R> FetchTodoByIdUseCase<'a, R>
where
    R: TodoRepository,
{
    pub fn new(repository: &'a R) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        FetchTodoById { id }: FetchTodoById<'_>,
    ) -> Result<Option<Todo>, FetchTodoByIdError> {
        let id = Id::try_from(id).map_err(FetchTodoByIdError::InvalidId)?;

        let query = FetchTodoByIdQuery::new(id);

        let todo = self
            .repository
            .fetch_todo_by_id(query)
            .await
            .map_err(FetchTodoByIdError::Repository)?;

        Ok(todo)
    }
}
