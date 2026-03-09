use time::OffsetDateTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date(OffsetDateTime);

impl Date {
    pub fn new(date: OffsetDateTime) -> Self {
        Self(date)
    }

    pub fn now_utc() -> Self {
        Self(OffsetDateTime::now_utc())
    }

    pub fn into_inner(self) -> OffsetDateTime {
        self.0
    }
}
