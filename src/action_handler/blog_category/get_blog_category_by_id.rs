use actix_web::web::Json;
use crate::db::get;
use crate::data_types::structs::Id;
use crate::data_types::traits::SerializeStruct;
use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use serde::Serialize;


pub async fn execute<T>(id: Json<Id>) -> Result<Json<T>, InternalError<String>>
where
    T: SerializeStruct + Serialize
{
    let res = get(T::name(), None, Some(vec!["id"]), Some(&[&id.id])).await;

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

// pub async fn execute(id: Json<Id>) -> String {
//     let mut data = vec![];

//     let res: Result<Vec<tokio_postgres::Row>, tokio_postgres::Error> = get("blog_category", None, Some(vec!["id"]), Some(&[&id.id])).await;

//     match res {
//         Ok(row) => {
//             if row.is_empty() {
//                 return format!(r#"{{"error": "row with id of {} not found"}}"#, &id.id);
//             }
//             data.push(BlogCategory {
//                 id: row[0].get(0),
//                 category: row[0].get(1),
//             });

//             let return_data: Return<Vec<BlogCategory>> = Return {data};

//             serde_json::to_string(&return_data).unwrap()
//         }
//         Err(e) => {
//             format!(r#"{{"error": "{}"}}"#, e)
//         }
//     }
// }

// from discord's harudagondi:
/*

impl Data for ProductFeature {
    fn name() -> &'static str { "product_feature" }
    fn from_row(row: tokio_postgres::Row) -> Self {
        ProductFeature {
            id: row[0].get(0),
            title: row[0].get(1),
            description: row[0].get(2),
        }
    }
}

pub async fn execute<T>(id: Json<Id>) -> Result<Json<T>, InternalError<String>>
where
    T: Data + Serialize
{
    let res = get(T::name(), None, Some(vec!["id"]), Some(&[&id.id])).await;

    let row = res.map_err(|e| {
        InternalError::New(
            e.to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    })?;

    if row.is_empty() {
        return Err(InternalError::new(
            format!("row with id of {} not found", id.id),
            StatusCode::NOT_FOUND
        ));
    }

    let data = T::from_row(row);

    Ok(Json(data));
}

// you can add the handlers like `execute::<ProductFeature>` and `execute::<BlogCategory>`
// there are some problems i've seen in some of ur code, i fixed some
// but not all
*/
