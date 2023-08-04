use crate::db::get_all;
use crate::data_types::structs::{ProductFeature, Return};

pub async fn execute() -> String {
    let mut data = vec![];

    let rows = get_all("product_feature").await;
    
    let mut i: usize = 0;
    loop {
        if i > rows.len() - 1 {
            break;
        }

        data.push(ProductFeature {
            id: rows[i].get(0),
            title: rows[i].get(1),
            description: rows[i].get(2),
        });
        
        i += 1;
    }

    let return_data: Return<Vec<ProductFeature>> = Return {data};

    serde_json::to_string(&return_data).unwrap()
}
