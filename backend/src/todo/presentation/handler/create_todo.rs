use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

use crate::todo::{
    application::{CreateTodo, CreateTodoError, CreateTodoUseCase},
    domain::Title,
    presentation::{
        common::{AppError, TodoDto},
        state::TodoState,
    },
};

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreateTodoRequest {
    title: String,
}

impl TryFrom<CreateTodoRequest> for CreateTodo {
    type Error = AppError;

    fn try_from(CreateTodoRequest { title }: CreateTodoRequest) -> Result<Self, Self::Error> {
        let title = Title::new(&title)?;

        Ok(CreateTodo::new(title))
    }
}

impl From<CreateTodoError> for AppError {
    fn from(value: CreateTodoError) -> Self {
        match value {
            CreateTodoError::Repository(error) => Self::internal(error),
        }
    }
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CreateTodoResponse {
    todo: TodoDto,
}

pub async fn create_todo(
    State(state): State<Arc<TodoState>>,
    Json(req): Json<CreateTodoRequest>,
) -> Result<impl IntoResponse, AppError> {
    let result = {
        let input = req.try_into()?;
        let db = state.db();
        CreateTodoUseCase::new(db).execute(input).await?
    };

    let response = CreateTodoResponse {
        todo: TodoDto::from(&result),
    };

    Ok((StatusCode::CREATED, Json(response)))
}
