use serde::{Serialize, Deserialize};
use std::fmt::{Display, Formatter, Result as FmtResult};
use crate::data_types::traits::PgPreparable;
use crate::data_types::traits::PgPreparable2;
use actix_web::web::Json;
use tokio_postgres::types::ToSql;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct ProductFeature {
    pub id: Option<i32>,
    pub title: String,
    pub description: String,
}

impl Display for ProductFeature {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let id = match self.id {
            Some(val) => val.to_string(),
            None => "None".to_string(),
        };

        write!(
            f,
            "ProductFeature [id: {}, title: {}, description: {}]",
            id,
            self.title,
            self.description
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewProductFeature {
    pub title: String,
    pub description: String,
}

impl PgPreparable for ProductFeature {
    fn name() -> &'static str {
        "product_feature"
    }

    fn new_from_row(row: &tokio_postgres::Row) -> Self {
        ProductFeature {
            id: row.get(0),
            title: row.get(1),
            description: row.get(2),
        }
    }

    fn columns() -> Vec<&'static str> {
        vec!["title", "description"]
    }

    fn values(&self) -> Vec<&(dyn ToSql + Sync + '_)> {
        let title: &String = &self.title;
        let description: &String = &self.description;

        vec![title, description]
    }
}

impl PgPreparable2 for ProductFeature {
    fn name() -> &'static str {
        "product_feature"
    }

   fn write_update_sql(update_body: &Self, id: String) -> String {
        format!("
            UPDATE
                product_feature
            SET
                title = '{}',
                description = '{}'
            WHERE
                id = {}
        ",
            update_body.title,
            update_body.description,
            id
        )
    }

    fn write_insert_sql(body: &Self) -> String {
        format!("
            INSERT INTO
                product_feature
                    (
                        title,
                        description
                    )
            VALUES
                title = '{}',
                description = '{}'
        ",
            body.title,
            body.description,
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
