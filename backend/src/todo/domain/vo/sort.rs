#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sort {
    field: SortField,
    order: SortOrder,
}

impl Sort {
    pub const fn new(field: SortField, order: SortOrder) -> Self {
        Self { field, order }
    }

    pub const fn asc(field: SortField) -> Self {
        Self {
            order: SortOrder::Ascending,
            field,
        }
    }

    pub const fn desc(field: SortField) -> Self {
        Self {
            order: SortOrder::Descending,
            field,
        }
    }

    pub fn field(&self) -> SortField {
        self.field
    }

    pub fn order(&self) -> SortOrder {
        self.order
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SortField {
    #[default]
    CreatedAt,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SortOrder {
    #[default]
    Descending,
    Ascending,
}
