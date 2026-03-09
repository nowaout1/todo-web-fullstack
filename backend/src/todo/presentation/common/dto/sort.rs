use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(
    Serialize, Deserialize, Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub struct SortDto {
    sort_field: Option<SortFieldDto>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    is_ascending: Option<bool>,
}

impl SortDto {
    pub fn sort_field(&self) -> Option<SortFieldDto> {
        self.sort_field
    }

    pub fn is_ascending(&self) -> Option<bool> {
        self.is_ascending
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "snake_case")]
pub enum SortFieldDto {
    CreatedAt,
}
