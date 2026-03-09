use thiserror::Error;

use crate::todo::{
    application::repository::{CreateTodoCommand, TodoRepository, TodoRepositoryError},
    domain::{entity::Todo, vo::Title},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreateTodo {
    title: Title,
}

impl CreateTodo {
    pub fn new(title: Title) -> Self {
        Self { title }
    }
}

#[derive(Error, Debug)]
pub enum CreateTodoError {
    #[error("failed to create todo: {0}")]
    Repository(#[from] TodoRepositoryError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreateTodoUseCase<'a, R>
where
    R: TodoRepository,
{
    repository: &'a R,
}

impl<'a, R> CreateTodoUseCase<'a, R>
where
    R: TodoRepository,
{
    pub fn new(repository: &'a R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, CreateTodo { title }: CreateTodo) -> Result<Todo, CreateTodoError> {
        let command = CreateTodoCommand::new(title);

        let todo = self
            .repository
            .create_todo(command)
            .await
            .map_err(CreateTodoError::Repository)?;

        Ok(todo)
    }
}
