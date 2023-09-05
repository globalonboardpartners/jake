use serde::{Serialize, Deserialize};
use crate::data_types::traits::PgPreparable;
use tokio_postgres::types::ToSql;

#[derive(Serialize, Deserialize, Debug)]
pub struct BlogCategory {
    pub id: i32,
    pub category: String,
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

// impl BlogCategory {
//     fn values(&self) -> Option<&'static [&'static (dyn ToSql + Sync + '_)]> where Self: std::marker::Sized + PgPreparable + Serialize {
//         let category: &String = &self.category;

//         Some(&[&category])
//     }
// }
