use thiserror::Error;

use crate::todo::{
    repository::{DeleteTodoByIdCommand, TodoRepository, TodoRepositoryError},
    vo::{Id, IdParseError},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeleteTodoById<'a> {
    id: &'a str,
}

impl<'a> DeleteTodoById<'a> {
    pub fn new(id: &'a str) -> Self {
        Self { id }
    }
    pub fn id(&self) -> &'a str {
        self.id
    }
}

#[derive(Error, Debug)]
pub enum DeleteTodoByIdError {
    #[error("got invalid todo id: {0}")]
    InvalidId(#[from] IdParseError),
    #[error("failed to delete todo: {0}")]
    Repository(#[from] TodoRepositoryError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeleteTodoByIdUseCase<'a, R> {
    repository: &'a R,
}

impl<'a, R> DeleteTodoByIdUseCase<'a, R>
where
    R: TodoRepository,
{
    pub fn new(repository: &'a R) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        DeleteTodoById { id }: DeleteTodoById<'_>,
    ) -> Result<(), DeleteTodoByIdError> {
        let id = Id::try_from(id).map_err(DeleteTodoByIdError::InvalidId)?;

        let command = DeleteTodoByIdCommand::new(id);

        self.repository
            .delete_todo_by_id(command)
            .await
            .map_err(DeleteTodoByIdError::Repository)?;

        Ok(())
    }
}
