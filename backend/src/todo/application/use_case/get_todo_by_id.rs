use thiserror::Error;

use crate::todo::{
    application::repository::{FetchTodoByIdQuery, TodoRepository, TodoRepositoryError},
    domain::{entity::Todo, vo::Id},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GetTodoById {
    id: Id,
}

impl GetTodoById {
    pub fn new(id: Id) -> Self {
        Self { id }
    }
}

#[derive(Error, Debug)]
pub enum GetTodoByIdError {
    #[error("failed to fetch todo by id: {0}")]
    Repository(#[from] TodoRepositoryError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetTodoByIdUseCase<'a, R>
where
    R: TodoRepository,
{
    repository: &'a R,
}

impl<'a, R> GetTodoByIdUseCase<'a, R>
where
    R: TodoRepository,
{
    pub fn new(repository: &'a R) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        GetTodoById { id }: GetTodoById,
    ) -> Result<Option<Todo>, GetTodoByIdError> {
        let query = FetchTodoByIdQuery::new(id);

        let todo = self
            .repository
            .fetch_todo_by_id(query)
            .await
            .map_err(GetTodoByIdError::Repository)?;

        Ok(todo)
    }
}
