use serde::Serialize;
use sqlx::PgPool;
use sqlx::postgres::PgRow;
use sqlx::postgres::PgQueryResult;
use sqlx::FromRow;
use std::env;
use crate::data_types::traits::PgPreparable2;
use actix_web::web::Json;

pub async fn connect_to_database() -> Result<PgPool, sqlx::Error> {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let pool = PgPool::connect(&database_url).await?;

    Ok(pool)
}

pub async fn get_all<T: 'static>() -> Result<Json<Vec<T>>, sqlx::Error>
where
    T: PgPreparable2 + Serialize + for<'r> FromRow<'r, PgRow> + Send + Unpin,
{
    let pg = connect_to_database().await;

    match pg {
        Ok(db) => {
            let query_string = format!("SELECT * FROM {}", T::name());
            let rows: Vec<T> = sqlx::query_as(&query_string)
                .fetch_all(&db)
                .await?;
            Ok(Json(rows))
        },
        Err(e) => Err(e)
    }
}

pub async fn get_by_id<T: 'static>(id: i32) -> Result<Json<T>, sqlx::Error>
where
    T: PgPreparable2 + Serialize + for<'r> FromRow<'r, PgRow> + Send + Unpin,
{
    let pg = connect_to_database().await;

    match pg {
        Ok(db) => {
            let query_string = format!("SELECT * FROM {} WHERE ID = {}", T::name(), id);
            let rows = sqlx::query_as(&query_string)
                .fetch_one(&db)
                .await?;
            Ok(Json(rows))
        },
        Err(e) => Err(e)
    }
}

pub async fn update_by_id<T: 'static>(update_body: Json<T>) -> Result<Json<PgQueryResult>, sqlx::Error>
where
    T: PgPreparable2 + Serialize + for<'r> FromRow<'r, PgRow> + Send + Unpin,
{
    let pg = connect_to_database().await;

    match pg {
        Ok(db) => {
            let query_string = format!("UPDATE {} {}", T::name(), T::prepare_update(Json(&update_body)));
            dbg!(&query_string);
            let rows = sqlx::query(&query_string)
                .execute(&db)
                .await?;
            Ok(Json(rows))
        },
        Err(e) => Err(e)
    }
}

pub async fn delete_by_id<T: 'static>(id: i32) -> Result<Json<PgQueryResult>, sqlx::Error>
where
    T: PgPreparable2 + Serialize + for<'r> FromRow<'r, PgRow> + Send + Unpin,
{
    let pg = connect_to_database().await;
    println!("in here");

    match pg {
        Ok(db) => {
            let query_string = format!("DELETE FROM {} WHERE id = {};", T::name(), id);
            dbg!(&query_string);
            let rows = sqlx::query(&query_string)
                .execute(&db)
                .await?;
            Ok(Json(rows))
        },
        Err(e) => Err(e)
    }
}
