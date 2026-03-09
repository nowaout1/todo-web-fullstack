use async_trait::async_trait;
use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::todo::{
    application::repository::{
        CreateTodoCommand, DeleteTodoByIdCommand, FetchRecentTodosQuery, FetchRecentTodosResponse,
        FetchTodoByIdQuery, SearchTodosQuery, SearchTodosResponse, TodoRepository,
        TodoRepositoryError,
    },
    domain::{Cursor, Limit, entity::Todo},
};
use dto::TodoDto;

mod dto;
mod error;

mod utils {
    use crate::todo::domain::{Cursor, Id, Todo};

    pub fn compute_next_cursor(todos: &[Todo], limit: impl Into<usize>) -> Option<Cursor> {
        let no_more = todos.len() < limit.into();

        if no_more {
            return None;
        }

        todos
            .last()
            .map(Todo::id)
            .map(Id::into_inner)
            .map(Cursor::new_unchecked)
    }
}

#[derive(Debug, Clone)]
pub struct TodoPostgres {
    pool: PgPool,
}

impl TodoPostgres {
    pub async fn new(addr: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(50)
            .connect(addr)
            .await?;

        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(Self { pool })
    }
}

#[async_trait]
impl TodoRepository for TodoPostgres {
    async fn fetch_todo_by_id(
        &self,
        input: FetchTodoByIdQuery,
    ) -> Result<Option<Todo>, TodoRepositoryError> {
        let id = input.id().as_usize();

        let query = sqlx::query_as!(
            TodoDto,
            r#"
                SELECT id, title, created_at
                FROM todos
                WHERE id = $1
                LIMIT 1
            "#,
            id as i64
        );

        let result = query
            .fetch_optional(&self.pool)
            .await?
            .as_ref()
            .map(TodoDto::to_entity);

        Ok(result)
    }

    async fn fetch_recent_todos(
        &self,
        input: FetchRecentTodosQuery,
    ) -> Result<FetchRecentTodosResponse, TodoRepositoryError> {
        let pagination = input.pagination();
        let cursor = pagination.cursor().map(Cursor::into_inner);
        let limit = pagination.limit().map(Limit::into_inner).unwrap_or(20);

        let todos = match cursor {
            Some(cursor) => {
                sqlx::query_as!(
                    TodoDto,
                    r#"
                    SELECT id, title, created_at
                    FROM todos
                    WHERE id < $1
                    ORDER BY id DESC
                    LIMIT $2
                "#,
                    cursor as i64,
                    limit as i64
                )
                .fetch_all(&self.pool)
                .await?
            }
            None => {
                sqlx::query_as!(
                    TodoDto,
                    r#"
                    SELECT id, title, created_at
                    FROM todos
                    ORDER BY id DESC
                    LIMIT $1
                "#,
                    limit as i64
                )
                .fetch_all(&self.pool)
                .await?
            }
        };

        let todos = todos.iter().map(TodoDto::to_entity).collect::<Vec<_>>();
        let next_cursor = utils::compute_next_cursor(&todos, limit);
        let response = FetchRecentTodosResponse::new(todos, next_cursor);

        Ok(response)
    }

    async fn search_todos(
        &self,
        input: SearchTodosQuery,
    ) -> Result<SearchTodosResponse, TodoRepositoryError> {
        let query = input.query().as_str();
        let pagination = input.pagination();
        let cursor = pagination.cursor().map(Cursor::into_inner);
        let limit = pagination.limit().map(Limit::into_inner).unwrap_or(20);

        let todos = match cursor {
            Some(cursor) => {
                sqlx::query_as!(
                    TodoDto,
                    r#"
                    SELECT id, title, created_at
                    FROM todos
                    WHERE id < $2
                    AND title ILIKE '%' || $1 || '%'
                    ORDER BY id DESC
                    LIMIT $3
                "#,
                    query,
                    cursor as i64,
                    limit as i64
                )
                .fetch_all(&self.pool)
                .await?
            }
            None => {
                sqlx::query_as!(
                    TodoDto,
                    r#"
                    SELECT id, title, created_at
                    FROM todos
                    WHERE title ILIKE '%' || $1 || '%'
                    ORDER BY id DESC
                    LIMIT $2
                "#,
                    query,
                    limit as i64
                )
                .fetch_all(&self.pool)
                .await?
            }
        };

        let todos = todos.iter().map(TodoDto::to_entity).collect::<Vec<_>>();
        let next_cursor = utils::compute_next_cursor(&todos, limit);
        let response = SearchTodosResponse::new(todos, next_cursor);

        Ok(response)
    }

    async fn create_todo(&self, input: CreateTodoCommand) -> Result<Todo, TodoRepositoryError> {
        let title = input.title().as_str();

        let query = sqlx::query_as!(
            TodoDto,
            r#"
                INSERT INTO todos (title)
                VALUES ($1)
                RETURNING id, title, created_at
            "#,
            title,
        );

        let result = query.fetch_one(&self.pool).await?.to_entity();

        Ok(result)
    }

    async fn delete_todo_by_id(
        &self,
        input: DeleteTodoByIdCommand,
    ) -> Result<(), TodoRepositoryError> {
        let id = input.id().as_usize();

        let query = sqlx::query!(
            r#"
                DELETE FROM todos
                WHERE id = $1
            "#,
            id as i64
        );

        query.execute(&self.pool).await?;

        Ok(())
    }
}
