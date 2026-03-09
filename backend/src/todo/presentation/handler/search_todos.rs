use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

use crate::todo::{
    application::{SearchTodos, SearchTodosError, SearchTodosUseCase},
    domain,
    presentation::{
        common::{AppError, PaginationDto, SortDto, dto::TodoDto},
        state::TodoState,
    },
};

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SearchTodosRequest {
    query: String,

    #[serde(flatten)]
    pagination: PaginationDto,

    #[serde(flatten)]
    sort: SortDto,
}

impl TryFrom<SearchTodosRequest> for SearchTodos {
    type Error = AppError;

    fn try_from(
        SearchTodosRequest {
            query,
            pagination,
            sort,
        }: SearchTodosRequest,
    ) -> Result<Self, Self::Error> {
        let query = domain::Query::new(&query)?;
        let pagination = pagination.try_into()?;
        let sort = sort.into();

        Ok(SearchTodos::new(query, pagination, sort))
    }
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SearchTodosResponse {
    todos: Vec<TodoDto>,
    next_cursor: Option<usize>,
    count: usize,
}

impl From<SearchTodosError> for AppError {
    fn from(value: SearchTodosError) -> Self {
        match value {
            SearchTodosError::Repository(error) => Self::internal(error),
        }
    }
}

pub async fn search_todos(
    Query(req): Query<SearchTodosRequest>,
    State(state): State<Arc<TodoState>>,
) -> Result<impl IntoResponse, AppError> {
    let result = {
        let input = req.try_into()?;
        SearchTodosUseCase::new(state.db()).execute(input).await?
    };

    let todos = {
        result
            .todos()
            .iter()
            .map(TodoDto::from)
            .collect::<Vec<TodoDto>>()
    };

    let count = todos.len();
    let next_cursor = result.next_cursor().map(|cursor| cursor.as_usize());

    let response = SearchTodosResponse {
        todos,
        next_cursor,
        count,
    };

    Ok((StatusCode::OK, Json(response)))
}
