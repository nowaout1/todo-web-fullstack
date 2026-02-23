use thiserror::Error;

use crate::todo::{
    entity::Todo,
    repository::{CreateTodoCommand, TodoRepository, TodoRepositoryError},
    vo::{Title, TitleError},
};

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreateTodo<'a> {
    title: &'a str,
}

impl<'a> CreateTodo<'a> {
    pub fn new(title: &'a str) -> Self {
        Self { title }
    }

    pub fn title(&self) -> &'a str {
        self.title
    }
}

#[derive(Error, Debug)]
pub enum CreateTodoError {
    #[error("invalid title: {0}")]
    InvalidTitle(#[from] TitleError),
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

    pub async fn execute(
        &self,
        CreateTodo { title }: CreateTodo<'_>,
    ) -> Result<Todo, CreateTodoError> {
        let title = Title::new(title).map_err(CreateTodoError::InvalidTitle)?;
        let todo = Todo::new(title);
        let command = CreateTodoCommand::new(todo);

        let todo = self
            .repository
            .create_todo(command)
            .await
            .map_err(CreateTodoError::Repository)?;

        Ok(todo)
    }
}
