use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(
    Serialize, Deserialize, Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub struct PaginationDto {
    #[serde_as(as = "Option<DisplayFromStr>")]
    cursor: Option<usize>,

    #[serde_as(as = "Option<DisplayFromStr>")]
    limit: Option<usize>,
}

impl PaginationDto {
    pub fn cursor(&self) -> Option<usize> {
        self.cursor
    }

    pub fn limit(&self) -> Option<usize> {
        self.limit
    }
}
