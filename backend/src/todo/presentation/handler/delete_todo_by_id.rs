use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::todo::{
    application::{DeleteTodoById, DeleteTodoByIdError, DeleteTodoByIdUseCase},
    domain::Id,
    presentation::{common::AppError, state::TodoState},
};

impl From<DeleteTodoByIdError> for AppError {
    fn from(value: DeleteTodoByIdError) -> Self {
        match value {
            DeleteTodoByIdError::Repository(error) => Self::internal(error),
        }
    }
}

pub async fn delete_todo_by_id(
    Path(id): Path<String>,
    State(state): State<Arc<TodoState>>,
) -> Result<impl IntoResponse, AppError> {
    let id = Id::try_from(id.as_str())?;

    DeleteTodoByIdUseCase::new(state.db())
        .execute(DeleteTodoById::new(id))
        .await?;

    Ok(StatusCode::OK)
}
