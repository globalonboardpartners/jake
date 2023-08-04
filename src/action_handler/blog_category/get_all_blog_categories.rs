use crate::db::get_all;
use crate::data_types::structs::{BlogCategory, Return};

pub async fn execute() -> String {
    let mut data = vec![];

    let rows = get_all("blog_category").await;
    
    let mut i: usize = 0;
    loop {
        if i > rows.len() - 1 {
            break;
        }

        data.push(BlogCategory {
            id: rows[i].get(0),
            category: rows[i].get(1),
        });
        
        i += 1;
    }

    let return_data: Return<Vec<BlogCategory>> = Return {data};

    serde_json::to_string(&return_data).unwrap()
}
