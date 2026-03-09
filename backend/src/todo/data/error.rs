use crate::todo::application::repository::TodoRepositoryError;

impl From<sqlx::Error> for TodoRepositoryError {
    fn from(value: sqlx::Error) -> Self {
        // TODO: refactor me

        let error = eyre::eyre!(value);
        TodoRepositoryError::Database(error)
    }
}
