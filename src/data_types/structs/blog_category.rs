use serde::{Serialize, Deserialize};
use crate::data_types::traits::PgPreparable;
use crate::data_types::traits::PgPreparable2;
use tokio_postgres::types::ToSql;
use actix_web::web::Json;
use std::fmt;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct BlogCategory {
    pub id: i32,
    pub category: String,
}

impl fmt::Debug for BlogCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BlogCategory {{ id: {}, category: \"{}\" }}", self.id, self.category)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewBlogCategory {
    pub category: String,
}

impl PgPreparable for BlogCategory {
    fn name() -> &'static str {
        "blog_category"
    }

    fn new_from_row(row: &tokio_postgres::Row) -> Self {
        BlogCategory { id: row.get(0), category: row.get(1) }
    }

    fn columns() -> Vec<&'static str> {
        vec!["category"]
    }

    fn values(&self) -> Vec<&(dyn ToSql + Sync + '_)> {
        let category: &String = &self.category;

        vec![category]
    }
}

impl PgPreparable2 for BlogCategory {
    fn name() -> &'static str {
        "blog_category"
    }

    fn prepare_update(update_body: Json<&BlogCategory>) -> String {
        format!(r#"SET category = '{}' WHERE id = {}"#, update_body.category, update_body.id)
    }
}
