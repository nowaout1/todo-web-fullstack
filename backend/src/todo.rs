use axum::{
    Router,
    routing::{delete, get, post},
};
use presentation::state::TodoState;
use std::sync::Arc;

use presentation::{create_todo, delete_todo_by_id, get_todo_by_id, get_recent_todos, search_todos};

pub mod application;
pub mod data;
pub mod domain;
pub mod presentation;

pub async fn create_todos_router() -> Router {
    let db_url =
        dotenvy::var("DATABASE_URL").expect("environment variable 'DATABASE_URL' not specified");
    let state = Arc::new(TodoState::new(&db_url).await);

    Router::new()
        .route("/todos/{id}", get(get_todo_by_id))
        .route("/todos", get(get_recent_todos))
        .route("/todos/search", get(search_todos))
        .route("/todos", post(create_todo))
        .route("/todos/{id}", delete(delete_todo_by_id))
        .with_state(state)
}
