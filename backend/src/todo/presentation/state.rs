use crate::todo::data::TodoPostgres;

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

    pub fn db(&self) -> &TodoPostgres {
        &self.db
    }
}
