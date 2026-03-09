use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

use crate::todo::{
    application::{GetRecentTodos, GetRecentTodosError, GetRecentTodosUseCase},
    presentation::{
        common::{AppError, PaginationDto, TodoDto},
        state::TodoState,
    },
};

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetTodosRequest {
    #[serde(flatten)]
    pagination: PaginationDto,
}

impl TryFrom<GetTodosRequest> for GetRecentTodos {
    type Error = AppError;

    fn try_from(GetTodosRequest { pagination }: GetTodosRequest) -> Result<Self, Self::Error> {
        let pagination = pagination.try_into()?;

        Ok(Self::new(pagination))
    }
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetTodosResponse {
    todos: Vec<TodoDto>,
    next_cursor: Option<usize>,
    count: usize,
}

impl From<GetRecentTodosError> for AppError {
    fn from(value: GetRecentTodosError) -> Self {
        match value {
            GetRecentTodosError::Repository(error) => Self::internal(error),
        }
    }
}

pub async fn get_recent_todos(
    Query(req): Query<GetTodosRequest>,
    State(state): State<Arc<TodoState>>,
) -> Result<impl IntoResponse, AppError> {
    let result = {
        let input = req.try_into()?;
        let db = state.db();
        GetRecentTodosUseCase::new(db).execute(input).await?
    };

    let todos = result.todos().iter().map(TodoDto::from).collect::<Vec<_>>();
    let count = todos.len();
    let next_cursor = result.next_cursor().map(|cursor| cursor.as_usize());

    let response = GetTodosResponse {
        todos,
        next_cursor,
        count,
    };

    Ok((StatusCode::OK, Json(response)))
}
