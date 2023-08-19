use crate::db::get_all;
use crate::data_types::structs::{Blog, Return};

pub async fn execute() -> String {
    let mut data = vec![];

    let rows = get_all("blog").await;
    
    let mut i: usize = 0;
    loop {
        if i > rows.len() - 1 {
            break;
        }

        data.push(Blog {
            id: rows[i].get(0),
            title: rows[i].get(1),
            slug: rows[i].get(2),
            category_id: rows[i].get(3),
            content: rows[i].get(4),
            image_link: rows[i].get(5),
            thumbnail_link: rows[i].get(6),
            publish_date: rows[i].get(7),
        });
        
        i += 1;
    }

    let return_data: Return<Vec<Blog>> = Return {data};

    serde_json::to_string(&return_data).unwrap()
}
