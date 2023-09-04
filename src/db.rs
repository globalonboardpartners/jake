use tokio_postgres::{NoTls, types::ToSql};
use dotenv::dotenv;
use std::env;
use actix_web::web::Json;
use crate::data_types::structs::Id;
use crate::data_types::traits::SerializeStruct;
use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use serde::Serialize;

pub struct QueryBuilder<'a> {
    query_str: String,
    // query params are not always needed. Sometimes you just want to do a simple select * from
    // x table.
    query_params: Option<&'a [&'a (dyn ToSql + Sync)]>
}

impl<'a> QueryBuilder<'a> {
    pub fn new(query_str: String, query_params: Option<&'a [&'a (dyn ToSql + Sync)]>) -> Self {
        Self {
            query_str,
            query_params
        }
    }

    pub async fn get(table: &str, columns: Option<Vec<&str>>, where_columns: Option<Vec<&str>>, where_values: Option<&'a [& (dyn ToSql + Sync)]>) -> Result<Vec<tokio_postgres::Row>, tokio_postgres::error::Error> {
        let mut columns_string: String = String::from("");
        let mut values_string: String = String::from("");
        let new_columns = match columns {
            Some(x) => x,
            None => vec!["*"]
        };
    
        for (_index, column) in new_columns.iter().enumerate() {
            columns_string = format!("{columns_string}, {}", column);
            values_string = format!("{values_string}, ${}", _index + 1);
        }
    
        let new_columns_string: String = columns_string.chars().skip(2).collect();
        let mut where_string: String = String::new();
    
        if let Some(x) = where_columns {
            for (_index, column) in x.iter().enumerate() {
                where_string.push_str(&format!("{} = ${}, ", column, _index + 1));
            }
        }
    
        let new_where_string = where_string.get(..where_string.len() - 2).unwrap_or("");
        let query_string = format!("SELECT {} FROM {} WHERE {};", new_columns_string, table, new_where_string);
        Self::query(Self::new(query_string, where_values)).await
    }

    pub async fn query(query: QueryBuilder<'_>) -> Result<Vec<tokio_postgres::Row>, tokio_postgres::error::Error> {
    
        dotenv().ok();
    
        let query_params = query.query_params.unwrap_or(&[]);
    
        let connection_str = format!(
            "host={} user={} port={} password={} dbname={}",
            get_env_var("DBHOST"),
            get_env_var("DBUSER"),
            get_env_var("DBPORT"),
            get_env_var("DBPASSWORD"),
            get_env_var("DBNAME")
        );
    
        let (client, connection) = tokio_postgres::connect(&connection_str, NoTls)
            .await
            .map_err(|e| {
                eprintln!("connection error: {}", e);
                e
        })?;
    
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });
    
        let rows = client
            .query(&query.query_str, query_params)
            .await?;
    
        Ok(rows)
    }

    pub async fn insert(table: &str, columns: Vec<&str>, values: Option<&'a [& (dyn ToSql + Sync)]>) -> Vec<tokio_postgres::Row> {
        let mut columns_string: String = String::from("");
        let mut values_string: String = String::from("");
        for (_index, column) in columns.iter().enumerate() {
            columns_string = format!("{columns_string}, {column}");
            values_string = format!("{values_string}, ${}", _index + 1);
        }
        let new_columns_string: String = columns_string.chars().skip(2).collect();
        let new_values_string: String = values_string.chars().skip(2).collect();
        let query_string = format!("INSERT INTO {} ({}) VALUES ({})", table, new_columns_string, new_values_string);
        Self::query(Self::new(query_string, values)).await.unwrap()
    }
    
    pub async fn get_all(table: &str) -> Vec<tokio_postgres::Row> {
        let query_string: String = format!("SELECT * FROM {};", table);
        Self::query(Self::new(query_string, None)).await.unwrap()
    }

    pub async fn delete(table: &str, where_columns: Option<Vec<&str>>, where_values: Option<&'a [& (dyn ToSql + Sync)]>) -> Vec<tokio_postgres::Row> {
        let mut where_string: String = String::new();
        if let Some(x) = where_columns {
            for (_index, column) in x.iter().enumerate() {
                where_string.push_str(&format!("{} = ${}, ", column, _index + 1));
            }
        }
        let mut query_string = format!("DELETE FROM {table} WHERE {}", where_string);
        query_string = query_string.trim_end_matches(", ").to_string();
        Self::query(Self::new(query_string, where_values)).await.unwrap()
    }

    pub async fn update(
        table: &str,
        set_columns: Option<Vec<&str>>,
        where_columns: Option<Vec<&str>>,
        values: Option<&'a [& (dyn ToSql + Sync)]>
    ) -> Vec<tokio_postgres::Row> {
        let mut set_string: String = String::new();
    
        let mut query_string = format!("UPDATE {}", table);
        let mut counter: i32 = 0;
        if let Some(x) = set_columns {
            // remove this unused variable or remove this warning
            for (_index, column) in x.iter().enumerate() {
                set_string.push_str(&format!("{}", column));
                counter += 1;
                set_string.push_str(&format!(" = ${counter}, "))
            }
        }
        set_string = set_string.trim_end_matches(", ").to_string();
        query_string = format!("{query_string} SET {}", set_string);
    
        let mut where_string: String = String::new();
    
        if let Some(x) = where_columns {
            // remove this unused variable or remove this warning
            for (_index, column) in x.iter().enumerate() {
                where_string.push_str(&format!("{}", column));
                counter += 1;
                where_string.push_str(&format!(" = ${counter}, "))
            }
        }
        where_string = where_string.trim_end_matches(", ").to_string();
        query_string = format!("{query_string} WHERE {}", where_string);
    
        println!("query_str: {:?}", query_string);
    
        Self::query(Self::new(query_string, values)).await.unwrap()
    }
}

fn get_env_var(env_var: &str) -> String {
    match env::var(env_var) {
        Ok(val) => val,
        Err(e) => {
            println!("{}", e);
            String::from("{e}")
        },
    }
}

pub async fn get_by_id<T>(id: Json<Id>) -> Result<Json<T>, InternalError<String>>
where
    T: SerializeStruct + Serialize
{
    let res = QueryBuilder::get(T::name(), None, Some(vec!["id"]), Some(&[&id.id])).await;

    let rows = res.map_err(|e| {
        InternalError::new(
            e.to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;

    if rows.is_empty() {
        return Err(InternalError::new(
            format!("row with id of {} not found", id.id),
            StatusCode::NOT_FOUND
        ));
    }

    let data = T::new_from_row(&rows[0]);

    Ok(Json(data))
}
