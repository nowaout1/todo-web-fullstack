use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Serialize;

use crate::todo::{
    application::{GetTodoById, GetTodoByIdError, GetTodoByIdUseCase},
    domain::Id,
    presentation::{
        common::{AppError, dto::TodoDto},
        state::TodoState,
    },
};

#[derive(Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GetTodoByIdResponse {
    todo: Option<TodoDto>,
}

impl From<GetTodoByIdError> for AppError {
    fn from(value: GetTodoByIdError) -> Self {
        match value {
            GetTodoByIdError::Repository(error) => Self::internal(error),
        }
    }
}

pub async fn get_todo_by_id(
    Path(id): Path<String>,
    State(state): State<Arc<TodoState>>,
) -> Result<impl IntoResponse, AppError> {
    let id = Id::try_from(id.as_str())?;

    let todo = GetTodoByIdUseCase::new(state.db())
        .execute(GetTodoById::new(id))
        .await?
        .as_ref()
        .map(TodoDto::from);

    let response = GetTodoByIdResponse { todo };

    Ok((StatusCode::OK, Json(response)))
}
