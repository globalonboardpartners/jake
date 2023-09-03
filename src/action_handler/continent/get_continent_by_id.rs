use actix_web::web::Json;
use crate::db::get;
use crate::data_types::structs::{Id, Continent, Return};

pub async fn execute(id: Json<Id>) -> String {
    let mut data = vec![];

    let res = get("continent", None, Some(vec!["id"]), Some(&[&id.id])).await;

    match res {
        Ok(row) => {
            if row.is_empty() {
                return format!(r#"{{"error": "row with id of {} not found"}}"#, &id.id);
            }
            data.push(Continent {
                id: row[0].get(0),
                name: row[0].get(1),
                slug: row[0].get(2),
                description_long: row[0].get(3),
                description_short: row[0].get(4),
                image_link: row[0].get(5),
                thumbnail_link: row[0].get(6),
                special_offer_image_link: row[0].get(7),
                video_link: row[0].get(8),
                gallery: row[0].get(9),
                tags: row[0].get(10),
                created: row[0].get(11),
                edited: row[0].get(12),
            });

            let return_data: Return<Vec<Continent>> = Return {data};

            serde_json::to_string(&return_data).unwrap()
        }
        Err(e) => {
            format!(r#"{{"error": "{}"}}"#, e)
        }
    }
}

// fn populate_struct_with_strings<T>(arr: &[String], mut my_struct: T) -> T
// where
//     T: Default + std::marker::Copy,
// {
//     // Make sure the array length matches the number of fields in the struct
//     assert_eq!(arr.len(), std::mem::size_of_val(&my_struct) / std::mem::size_of::<String>());

//     // Iterate over the array and set the values in the struct
//     for (i, field) in arr.iter().enumerate() {
//         match i {
//             0 => my_struct.field1 = field.clone(),
//             1 => my_struct.field2 = field.clone(),
//             2 => my_struct.field3 = field.clone(),
//             // Add more fields as needed
//             _ => {}
//         }
//     }

//     my_struct
// }

// pub fn make_json_string<T>(response: Result<Vec<tokio_postgres::Row>, tokio_postgres::error::Error>, custom_struct: T) -> String {
//     let mut data = vec![];

//     match response {
//         Ok(row) => {
//             data.push(Continent {
//                 id: row[0].get(0),
//                 name: row[0].get(1),
//                 slug: row[0].get(2),
//                 description_long: row[0].get(3),
//                 description_short: row[0].get(4),
//                 image_link: row[0].get(5),
//                 thumbnail_link: row[0].get(6),
//                 special_offer_image_link: row[0].get(7),
//                 video_link: row[0].get(8),
//                 gallery: row[0].get(9),
//                 tags: row[0].get(10),
//                 created: row[0].get(11),
//                 edited: row[0].get(12),
//             });

//             let return_data: Return<Vec<Continent>> = Return {data};

//             serde_json::to_string(&return_data).unwrap()
//         }
//         Err(e) => {
//             format!(r#"{{"error": "{}"}}"#, e)
//         }
//     }
// }

// take a struct
// loop through it's feilds, reading them into an array
// take an array of values
// ["id", "title", "content"]
// [1, "hi", "yadada..."]
// let mut i: i32 = 0;
// loop {
//      if i >= arr.len() {
//          break;
//      }
//
//      structT {}
//
// i += 1;
// }
// and then do something like struct.
//
// one thing I could do is create a "new" func for each struct. That way I could have a way to
// easily initialize an empty struct. then I would 
//

// struct MyStruct {
//     name: String,
//     age: i32
// }

// pub fn new_dyn<T,U>(struct_name: T, vals: Vec<U>) {
//     // loop through given struct, collect all field names in array
//     // take the array of given vals
//     // loop through the struct feilds array and then dynamically access and set the 
// }

// also: it is pretty normal to have a "new" function on structs. Maybe I should just have the new
// function take in an array of vals and then hook them all up appropriately...
//
// that being said, I still think it would be nice to have a function built into structs that would
// do this automatically for me...
// 
//
//
//


