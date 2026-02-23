use async_trait::async_trait;
use thiserror::Error;

use crate::todo::{
    entity::Todo,
    vo::{Date, Id, Limit, Offset, Query, Title},
};

pub type FetchOffset = Offset<0, { usize::MAX }>;
pub type FetchLimit = Limit<1, 32>;

#[derive(Error, Debug)]
pub enum TodoRepositoryError {
    #[error("not found: {0}")]
    NotFound(#[source] eyre::Error),

    #[error("todo already exists: {0}")]
    AlreadyExists(String),

    #[error("unauthorized: {0}")]
    Unauthorized(String),

    #[error("database constraint violation: {0}")]
    ConstraintViolation(String),

    #[error("database error: {0}")]
    Database(#[source] eyre::Error),

    #[error("connection was interrupted: {0}")]
    Connection(#[source] eyre::Error),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FetchTodoByIdQuery {
    id: Id,
}

impl FetchTodoByIdQuery {
    pub fn new(id: Id) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FetchRecentTodosQuery {
    offset: FetchOffset,
    limit: FetchLimit,
}

impl FetchRecentTodosQuery {
    pub fn new(offset: FetchOffset, limit: FetchLimit) -> Self {
        Self { offset, limit }
    }

    pub fn offset(&self) -> &FetchOffset {
        &self.offset
    }

    pub fn limit(&self) -> &FetchLimit {
        &self.limit
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SearchTodosQuery {
    query: Query,
    offset: FetchOffset,
    limit: FetchLimit,
}

impl SearchTodosQuery {
    pub fn new(query: Query, offset: FetchOffset, limit: FetchLimit) -> Self {
        Self {
            query,
            offset,
            limit,
        }
    }

    pub fn query(&self) -> &Query {
        &self.query
    }

    pub fn offset(&self) -> &FetchOffset {
        &self.offset
    }

    pub fn limit(&self) -> &FetchLimit {
        &self.limit
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreateTodoCommand {
    todo: Todo,
}

impl CreateTodoCommand {
    pub fn new(todo: Todo) -> Self {
        Self { todo }
    }

    pub fn todo(&self) -> &Todo {
        &self.todo
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeleteTodoByIdCommand {
    id: Id,
}

impl DeleteTodoByIdCommand {
    pub fn new(id: Id) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &Id {
        &self.id
    }
}

#[async_trait]
pub trait TodoRepository {
    async fn fetch_todo_by_id(
        &self,
        input: FetchTodoByIdQuery,
    ) -> Result<Option<Todo>, TodoRepositoryError>;
    async fn fetch_recent_todos(
        &self,
        input: FetchRecentTodosQuery,
    ) -> Result<Vec<Todo>, TodoRepositoryError>;
    async fn search_todos(&self, input: SearchTodosQuery)
    -> Result<Vec<Todo>, TodoRepositoryError>;
    async fn create_todo(&self, input: CreateTodoCommand) -> Result<Todo, TodoRepositoryError>;
    async fn delete_todo_by_id(
        &self,
        input: DeleteTodoByIdCommand,
    ) -> Result<(), TodoRepositoryError>;
}
