use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct BlogCategory {
    pub id: Option<i32>,
    pub category: String,
}

impl std::fmt::Debug for BlogCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BlogCategory {{ id: {:?}, category: \"{}\" }}",
            self.id, self.category
        )
    }
}
