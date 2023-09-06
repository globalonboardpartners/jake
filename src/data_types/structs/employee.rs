use serde::{Serialize, Deserialize};
use crate::data_types::traits::PgPreparable;
use crate::data_types::traits::PgPreparable2;
use tokio_postgres::types::ToSql;
use actix_web::web::Json;
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Employee {
    pub id: Option<i32>,
    pub name: String,
    pub position: String,
    pub bio: String,
    pub image_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewEmployee {
    pub name: String,
    pub position: String,
    pub bio: String,
    pub image_url: String,
}

impl PgPreparable for Employee {
    fn new_from_row(row: &tokio_postgres::Row) -> Self {
        Employee {
            id: row.get(0),
            name: row.get(1),
            position: row.get(2),
            bio: row.get(3),
            image_url: row.get(4),
        }
    }
    fn name() -> &'static str {
        "employee"
    }

    fn columns() -> Vec<&'static str> {
        vec!["name", "position", "bio", "image_url"]
    }

    fn values(&self) -> Vec<&(dyn ToSql + Sync + '_)> {
        let name: &String = &self.name;
        let position: &String = &self.position;
        let bio: &String = &self.bio;
        let image_url: &String = &self.image_url;

        vec![
            name,
            position,
            bio,
            image_url,
        ]
    }
}

impl PgPreparable2 for Employee {
    fn name() -> &'static str {
        "employee"
    }

    fn write_update_sql(update_body: &Self, id: String) -> String {
        format!("
            UPDATE
                employee
            SET
                name = '{}',
                position = '{}',
                bio = '{}',
                image_url = '{}'
            WHERE
                id = {}
        ",
            update_body.name,
            update_body.position,
            update_body.bio,
            update_body.image_url,
            id
        )
    }

    fn write_insert_sql(body: &Self) -> String {
        format!("
            INSERT INTO
                blog_category
                    (
                        name,
                        position,
                        bio,
                        image_url
                    )
            VALUES
                name = '{}',
                position = '{}',
                bio = '{}',
                image_url = '{}'
        ",
            body.name,
            body.position,
            body.bio,
            body.image_url,
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
