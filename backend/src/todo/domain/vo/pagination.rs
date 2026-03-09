pub mod cursor;
pub mod limit;

pub use cursor::{Cursor, CursorError};
pub use limit::{Limit, LimitError};

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pagination {
    cursor: Option<Cursor>,
    limit: Option<Limit>,
}

impl Pagination {
    pub fn new(cursor: Option<Cursor>, limit: Option<Limit>) -> Self {
        Self { cursor, limit }
    }

    pub fn cursor(&self) -> Option<Cursor> {
        self.cursor
    }

    pub fn limit(&self) -> Option<Limit> {
        self.limit
    }
}
