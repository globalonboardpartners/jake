use serde::{Serialize, Deserialize};
use crate::data_types::traits::PgPreparable;
use crate::data_types::traits::PgPreparable2;
use tokio_postgres::types::ToSql;
use actix_web::web::Json;
use std::fmt;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct BlogCategory {
    pub id: Option<i32>,
    pub category: String,
}

impl fmt::Debug for BlogCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BlogCategory {{ id: {:?}, category: \"{}\" }}", self.id, self.category)
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

    fn write_update_sql(update_body: &Self, id: String) -> String {
        format!("
            UPDATE
                blog_category
            SET
                category = '{}'
            WHERE
                id = {}
        ",
            update_body.category,
            id
        )
    }

    fn write_insert_sql(body: &Self) -> String {
        format!("
            INSERT INTO
                blog_category
                    (category)
            VALUES
                category = '{}'
        ",
            body.category,
        )
    }

    fn id(&self) -> Option<i32> {
        self.id
    }

    fn into_id(&self) -> String {
        match &self.id {
            Some(x) => x.to_string(),
            None => "None".to_string()
        }
    }
}
