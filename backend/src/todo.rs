use crate::todo::{
    entity::Todo,
    use_case::{
        CreateTodo, CreateTodoError, CreateTodoUseCase, DeleteTodoById, DeleteTodoByIdError,
        DeleteTodoByIdUseCase, FetchRecentTodos, FetchRecentTodosError, FetchRecentTodosUseCase,
        FetchTodoById, FetchTodoByIdError, FetchTodoByIdUseCase, SearchTodos, SearchTodosError,
        SearchTodosUseCase,
    },
};

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub mod data;
pub mod entity;
pub mod repository;
pub mod use_case;
pub mod vo;

pub use data::TodoPostgres;

#[derive(Clone)]
pub struct TodoState {
    db: TodoPostgres,
}

impl TodoState {
    pub async fn new(db_url: &str) -> Self {
        let db = TodoPostgres::new(&db_url)
            .await
            .expect("failed to connect database");

        Self { db }
    }
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TodoDto {
    id: String,
    title: String,
    created_at: String,
}

impl TodoDto {
    pub fn from_entity(entity: &Todo) -> Self {
        Self {
            id: entity.id().into_inner().to_string(),
            title: entity.title().clone().into_inner(),
            created_at: entity.created_at().into_inner().to_string(),
        }
    }
}

pub async fn create_todos_router() -> Router {
    let db_url =
        dotenvy::var("DATABASE_URL").expect("environment variable 'DATABASE_URL' not specified");

    let state = Arc::new(TodoState::new(&db_url).await);

    Router::new()
        .route("/todos/{id}", get(fetch_todo_by_id))
        .route("/todos", get(fetch_recent_todos))
        .route("/todos/search", get(search_todos))
        .route("/todos", post(create_todo))
        .route("/todos/{id}", delete(delete_todo_by_id))
        .with_state(state)
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FetchTodoByIdResponse {
    todo: Option<TodoDto>,
}

pub async fn fetch_todo_by_id(
    Path(id): Path<String>,
    State(state): State<Arc<TodoState>>,
) -> impl IntoResponse {
    let result = FetchTodoByIdUseCase::new(&state.db)
        .execute(FetchTodoById::new(&id))
        .await;

    let code = match result {
        Ok(_) => StatusCode::OK,
        Err(FetchTodoByIdError::InvalidId(_)) => StatusCode::BAD_REQUEST,
        Err(FetchTodoByIdError::Repository(_)) => StatusCode::INTERNAL_SERVER_ERROR,
    };

    let todo = result.ok().flatten().as_ref().map(TodoDto::from_entity);
    let response = FetchTodoByIdResponse { todo };

    (code, Json(response))
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FetchRecentTodosRequest {
    offset: i32,
    limit: Option<i32>,
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FetchRecentTodosResponse {
    todos: Vec<TodoDto>,
    count: usize,
}

pub async fn fetch_recent_todos(
    Query(FetchRecentTodosRequest { offset, limit }): Query<FetchRecentTodosRequest>,
    State(state): State<Arc<TodoState>>,
) -> impl IntoResponse {
    let result = FetchRecentTodosUseCase::new(&state.db)
        .execute(FetchRecentTodos::new(offset, limit))
        .await;

    let code = match result {
        Ok(_) => StatusCode::OK,
        Err(FetchRecentTodosError::Limit(_)) => StatusCode::BAD_REQUEST,
        Err(FetchRecentTodosError::Offset(_)) => StatusCode::BAD_REQUEST,
        Err(FetchRecentTodosError::Repository(_)) => StatusCode::INTERNAL_SERVER_ERROR,
    };

    let todos = result
        .unwrap_or_default()
        .iter()
        .map(TodoDto::from_entity)
        .collect::<Vec<TodoDto>>();

    let count = todos.len();

    let response = FetchRecentTodosResponse { todos, count };

    (code, Json(response))
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SearchTodosRequest {
    query: String,
    offset: Option<i32>,
    limit: Option<i32>,
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SearchTodosResponse {
    todos: Vec<TodoDto>,
    count: usize,
}

pub async fn search_todos(
    Query(SearchTodosRequest {
        query,
        offset,
        limit,
    }): Query<SearchTodosRequest>,
    State(state): State<Arc<TodoState>>,
) -> impl IntoResponse {
    let result = SearchTodosUseCase::new(&state.db)
        .execute(SearchTodos::new(&query, offset, limit))
        .await;

    let code = match result {
        Ok(_) => StatusCode::OK,
        Err(SearchTodosError::Query(_)) => StatusCode::BAD_REQUEST,
        Err(SearchTodosError::Limit(_)) => StatusCode::BAD_REQUEST,
        Err(SearchTodosError::Offset(_)) => StatusCode::BAD_REQUEST,
        Err(SearchTodosError::Repository(_)) => StatusCode::INTERNAL_SERVER_ERROR,
    };

    let todos = result
        .unwrap_or_default()
        .iter()
        .map(TodoDto::from_entity)
        .collect::<Vec<TodoDto>>();

    let count = todos.len();

    let response = SearchTodosResponse { todos, count };

    (code, Json(response))
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreateTodoRequest {
    title: String,
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreateTodoResponse {
    todo: Option<TodoDto>,
}

pub async fn create_todo(
    Query(CreateTodoRequest { title }): Query<CreateTodoRequest>,
    State(state): State<Arc<TodoState>>,
) -> impl IntoResponse {
    let result = CreateTodoUseCase::new(&state.db)
        .execute(CreateTodo::new(&title))
        .await;

    let code = match result {
        Ok(_) => StatusCode::CREATED,
        Err(CreateTodoError::InvalidTitle(_)) => StatusCode::BAD_REQUEST,
        Err(CreateTodoError::Repository(_)) => StatusCode::INTERNAL_SERVER_ERROR,
    };

    let response = CreateTodoResponse {
        todo: result.ok().as_ref().map(TodoDto::from_entity),
    };

    (code, Json(response))
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeleteTodoByIdResponse {}

pub async fn delete_todo_by_id(
    Path(id): Path<String>,
    State(state): State<Arc<TodoState>>,
) -> impl IntoResponse {
    let result = DeleteTodoByIdUseCase::new(&state.db)
        .execute(DeleteTodoById::new(&id))
        .await;

    let code = match result {
        Ok(_) => StatusCode::OK,
        Err(DeleteTodoByIdError::InvalidId(_)) => StatusCode::BAD_REQUEST,
        Err(DeleteTodoByIdError::Repository(_)) => StatusCode::INTERNAL_SERVER_ERROR,
    };

    let response = DeleteTodoByIdResponse {};

    (code, Json(response))
}
