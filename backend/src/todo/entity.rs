use crate::todo::vo::{Date, Id, Title};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Todo {
    id: Id,
    title: Title,
    created_at: Date,
}

impl Todo {
    pub fn new(title: Title) -> Self {
        let id = Id::new();
        let created_at = Date::now_utc();

        Self {
            id,
            title,
            created_at,
        }
    }

    pub fn from_parts(id: Id, title: Title, created_at: Date) -> Self {
        Self {
            id,
            title,
            created_at,
        }
    }

    pub fn id(&self) -> Id {
        self.id
    }

    pub fn title(&self) -> &Title {
        &self.title
    }

    pub fn created_at(&self) -> &Date {
        &self.created_at
    }
}
