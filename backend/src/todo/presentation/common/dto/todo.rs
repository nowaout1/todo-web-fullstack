use serde::Serialize;
use time::OffsetDateTime;

#[derive(Serialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TodoDto {
    id: String,
    title: String,
    #[serde(with = "time::serde::rfc3339")]
    created_at: OffsetDateTime,
}

impl TodoDto {
    pub fn new(id: String, title: String, created_at: OffsetDateTime) -> Self {
        Self {
            id,
            title,
            created_at,
        }
    }
}
