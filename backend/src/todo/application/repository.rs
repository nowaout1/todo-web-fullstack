use async_trait::async_trait;
use thiserror::Error;

use crate::todo::domain::{
    Cursor, Title,
    entity::Todo,
    vo::{Id, Pagination, Query, Sort},
};

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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreateTodoCommand {
    title: Title,
}

impl CreateTodoCommand {
    pub fn new(title: Title) -> Self {
        Self { title }
    }

    pub fn title(&self) -> &Title {
        &self.title
    }
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
    pagination: Pagination,
}

impl FetchRecentTodosQuery {
    pub fn new(pagination: Pagination) -> Self {
        Self { pagination }
    }

    pub fn pagination(&self) -> Pagination {
        self.pagination
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FetchRecentTodosResponse {
    todos: Vec<Todo>,
    next_cursor: Option<Cursor>,
}

impl FetchRecentTodosResponse {
    pub fn new(todos: Vec<Todo>, next_cursor: Option<Cursor>) -> Self {
        Self { todos, next_cursor }
    }

    pub fn todos(&self) -> &[Todo] {
        &self.todos[..]
    }

    pub fn next_cursor(&self) -> Option<&Cursor> {
        self.next_cursor.as_ref()
    }

    pub fn has_next(&self) -> bool {
        self.next_cursor().is_some()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SearchTodosQuery {
    query: Query,
    pagination: Pagination,
    sort: Sort,
}

impl SearchTodosQuery {
    pub fn new(query: Query, pagination: Pagination, sort: Sort) -> Self {
        Self {
            query,
            pagination,
            sort,
        }
    }

    pub fn query(&self) -> &Query {
        &self.query
    }

    pub fn pagination(&self) -> Pagination {
        self.pagination
    }

    pub fn sort(&self) -> Sort {
        self.sort
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
    ) -> Result<FetchRecentTodosResponse, TodoRepositoryError>;
    async fn search_todos(
        &self,
        input: SearchTodosQuery,
    ) -> Result<SearchTodosResponse, TodoRepositoryError>;
    async fn create_todo(&self, input: CreateTodoCommand) -> Result<Todo, TodoRepositoryError>;
    async fn delete_todo_by_id(
        &self,
        input: DeleteTodoByIdCommand,
    ) -> Result<(), TodoRepositoryError>;
}
