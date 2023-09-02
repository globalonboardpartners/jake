use crate::db::get_all;
use crate::data_types::structs::{Blog, Return};
use crate::utils::format_unix_timestamp;

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
            featured: rows[i].get(7),
            publish_date: format_unix_timestamp(rows[i].get(8), None),
        });
        
        i += 1;
    }

    let return_data: Return<Vec<Blog>> = Return {data};

    serde_json::to_string(&return_data).unwrap()
}
