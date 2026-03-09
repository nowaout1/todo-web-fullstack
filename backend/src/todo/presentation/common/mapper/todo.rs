use crate::todo::{domain::Todo, presentation::common::TodoDto};

impl From<&Todo> for TodoDto {
    fn from(value: &Todo) -> Self {
        let id = value.id().as_usize().to_string();
        let title = value.title().clone().into_inner();
        let created_at = value.created_at().into_inner();

        Self::new(id, title, created_at)
    }
}
