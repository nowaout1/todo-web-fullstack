use crate::todo::{
    domain::{Sort, SortField, SortOrder},
    presentation::common::{SortDto, SortFieldDto},
};

impl From<SortDto> for Sort {
    fn from(value: SortDto) -> Self {
        let field = match value.sort_field() {
            Some(SortFieldDto::CreatedAt) => SortField::CreatedAt,
            None => SortField::default(),
        };

        let order = match value.is_ascending() {
            Some(true) => SortOrder::Ascending,
            Some(false) => SortOrder::Descending,
            None => SortOrder::default(),
        };

        Sort::new(field, order)
    }
}
