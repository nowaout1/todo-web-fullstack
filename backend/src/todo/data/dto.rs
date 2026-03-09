use time::OffsetDateTime;

use crate::todo::domain::{Date, Id, Title, Todo};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TodoDto {
    pub id: i64,
    pub title: String,
    pub created_at: OffsetDateTime,
}

impl TodoDto {
    pub fn to_entity(&self) -> Todo {
        let id = Id::from_signed_integer(self.id);
        let title = Title::new_unchecked(&self.title);
        let created_at = Date::new(self.created_at);

        Todo::new(id, title, created_at)
    }
}
