use async_trait::async_trait;
use sqlx::{PgPool, postgres::PgPoolOptions, types::time::OffsetDateTime};

use crate::todo::{
    entity::Todo,
    repository::{
        CreateTodoCommand, DeleteTodoByIdCommand, FetchRecentTodosQuery, FetchTodoByIdQuery,
        SearchTodosQuery, TodoRepository, TodoRepositoryError,
    },
    vo::{Date, Id, Title},
};

impl From<sqlx::Error> for TodoRepositoryError {
    fn from(value: sqlx::Error) -> Self {
        // TODO: refactor me

        let error = eyre::eyre!(value);
        TodoRepositoryError::Database(error)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TodoDto {
    id: String,
    title: String,
    created_at: OffsetDateTime,
}

impl TodoDto {
    pub fn to_entity(&self) -> Todo {
        let id = Id::new_unchecked(&self.id);
        let title = Title::new_unchecked(&self.title);
        let created_at = Date::new(self.created_at);

        Todo::from_parts(id, title, created_at)
    }
}

#[derive(Debug, Clone)]
pub struct TodoPostgres {
    pool: PgPool,
}

impl TodoPostgres {
    pub async fn new(addr: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(addr)
            .await?;

        Ok(Self { pool })
    }
}

#[async_trait]
impl TodoRepository for TodoPostgres {
    async fn fetch_todo_by_id(
        &self,
        input: FetchTodoByIdQuery,
    ) -> Result<Option<Todo>, TodoRepositoryError> {
        let id = input.id().into_inner();

        let query = sqlx::query_as!(
            TodoDto,
            r#"
                SELECT id, title, created_at
                FROM todos
                WHERE id = $1
                LIMIT 1
            "#,
            id
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
    ) -> Result<Vec<Todo>, TodoRepositoryError> {
        let offset = input.offset().into_inner();
        let limit = input.limit().into_inner();

        let query = sqlx::query_as!(
            TodoDto,
            r#"
                SELECT id, title, created_at
                FROM todos
                ORDER BY created_at DESC
                OFFSET $1
                LIMIT $2
            "#,
            offset as i32,
            limit as i32
        );

        let result = query
            .fetch_all(&self.pool)
            .await?
            .iter()
            .map(TodoDto::to_entity)
            .collect();

        Ok(result)
    }

    async fn search_todos(
        &self,
        input: SearchTodosQuery,
    ) -> Result<Vec<Todo>, TodoRepositoryError> {
        let query = input.query().as_str();
        let offset = input.offset().into_inner();
        let limit = input.limit().into_inner();

        let query = sqlx::query_as!(
            TodoDto,
            r#"
                SELECT id, title, created_at
                FROM todos
                WHERE title ILIKE '%' || $1 || '%'
                ORDER BY created_at DESC
                OFFSET $2
                LIMIT $3
            "#,
            query,
            offset as i32,
            limit as i32
        );

        let result = query
            .fetch_all(&self.pool)
            .await?
            .iter()
            .map(TodoDto::to_entity)
            .collect();

        Ok(result)
    }

    async fn create_todo(&self, input: CreateTodoCommand) -> Result<Todo, TodoRepositoryError> {
        let todo = input.todo();
        let id = todo.id().into_inner();
        let title = todo.title().to_string();
        let created_at = todo.created_at().into_inner();

        let query = sqlx::query_as!(
            TodoDto,
            r#"
                INSERT INTO todos (id, title, created_at)
                VALUES ($1, $2, $3)
                RETURNING id, title, created_at
            "#,
            id,
            title,
            created_at
        );

        let result = query.fetch_one(&self.pool).await?.to_entity();

        Ok(result)
    }

    async fn delete_todo_by_id(
        &self,
        input: DeleteTodoByIdCommand,
    ) -> Result<(), TodoRepositoryError> {
        let id = input.id().into_inner();

        let query = sqlx::query!(
            r#"
                DELETE FROM todos
                WHERE id = $1
            "#,
            id
        );

        query.execute(&self.pool).await?;

        Ok(())
    }
}
