use thiserror::Error;

use crate::todo::{
    application::repository::{DeleteTodoByIdCommand, TodoRepository, TodoRepositoryError},
    domain::vo::Id,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeleteTodoById {
    id: Id,
}

impl DeleteTodoById {
    pub fn new(id: Id) -> Self {
        Self { id }
    }
}

#[derive(Error, Debug)]
pub enum DeleteTodoByIdError {
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
        DeleteTodoById { id }: DeleteTodoById,
    ) -> Result<(), DeleteTodoByIdError> {
        let command = DeleteTodoByIdCommand::new(id);

        self.repository
            .delete_todo_by_id(command)
            .await
            .map_err(DeleteTodoByIdError::Repository)?;

        Ok(())
    }
}
