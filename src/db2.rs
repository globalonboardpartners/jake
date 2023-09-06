use sqlx::PgPool;
use std::env;
use sqlx::Error;
use crate::data_types::types::ErrorMessage;

pub async fn connect() -> Result<PgPool, ErrorMessage> {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let pool = PgPool::connect(&database_url).await;

    match pool {
        Ok(db) => Ok(db),
        Err(e) => match e {
            Error::Configuration(e) => Err(format!("Configuration error: {}", e)),
            Error::Database(e) => Err(format!("Database error: {}", e)),
            Error::Database(e) => Err(format!("Database error: {}", e)),
            Error::Database(e) => Err(format!("Database error: {}", e)),
            _ => Err(format!("Unknown Error: {}", e))
        }
    }
}

// pub async fn create<T: 'static>(body: T) -> Result<Json<PgQueryResult>, sqlx::Error>
// where
//     T: PgPreparable2 + Serialize + for<'r> FromRow<'r, PgRow> + Send + Unpin,
// {
//     let pg = connect().await;

//     match pg {
//         Ok(db) => {
//             let mut query_string = T::write_insert_sql(&body);
//             query_string = query_string.replace("\n", "");
//             let rows = sqlx::query(&query_string)
//                 .execute(&db)
//                 .await?;
//             Ok(Json(rows))
//         },
//         Err(e) => Err(e)
//     }
// }

// pub async fn get_all<T: 'static>() -> Result<Json<Vec<T>>, sqlx::Error>
// where
//     T: PgPreparable2 + Serialize + for<'r> FromRow<'r, PgRow> + Send + Unpin,
// {
//     let pg = connect().await;

//     match pg {
//         Ok(db) => {
//             let query_string = format!("SELECT * FROM {}", T::name());
//             let rows: Vec<T> = sqlx::query_as(&query_string)
//                 .fetch_all(&db)
//                 .await?;
//             Ok(Json(rows))
//         },
//         Err(e) => Err(e)
//     }
// }

// pub async fn get_by_id<T: 'static>(id: i32) -> Result<Json<T>, sqlx::Error>
// where
//     T: PgPreparable2 + Serialize + for<'r> FromRow<'r, PgRow> + Send + Unpin,
// {
//     let pg = connect().await;

//     match pg {
//         Ok(db) => {
//             let query_string = format!("SELECT * FROM {} WHERE ID = {}", T::name(), id);
//             let rows = sqlx::query_as(&query_string)
//                 .fetch_one(&db)
//                 .await?;
//             Ok(Json(rows))
//         },
//         Err(e) => Err(e)
//     }
// }

// pub async fn update_by_id<T: 'static>(update_body: T) -> Result<Json<PgQueryResult>, sqlx::Error>
// where
//     T: PgPreparable2 + Serialize + for<'r> FromRow<'r, PgRow> + Send + Unpin,
// {
//     let pg = connect().await;

//     match pg {
//         Ok(db) => {
//             let mut query_string = T::write_update_sql(&update_body, update_body.into_id());
//             query_string = query_string.replace("\n", "");
//             let rows = sqlx::query(&query_string)
//                 .execute(&db)
//                 .await?;
//             Ok(Json(rows))
//         },
//         Err(e) => Err(e)
//     }
// }

// pub async fn delete_by_id<T: 'static>(id: i32) -> Result<Json<PgQueryResult>, sqlx::Error>
// where
//     T: PgPreparable2 + Serialize + for<'r> FromRow<'r, PgRow> + Send + Unpin,
// {
//     let pg = connect().await;

//     match pg {
//         Ok(db) => {
//             let query_string = format!("DELETE FROM {} WHERE id = {};", T::name(), id);
//             let rows = sqlx::query(&query_string)
//                 .execute(&db)
//                 .await?;
//             Ok(Json(rows))
//         },
//         Err(e) => Err(e)
//     }
// }
