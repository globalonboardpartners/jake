use crate::db::get_all;
use crate::data_types::structs::{Continent, Return};

pub async fn execute() -> String {
    let mut data = vec![];

    let rows = get_all("continent").await;
    
    let mut i: usize = 0;
    loop {
        if i > rows.len() - 1 {
            break;
        }

        data.push(Continent {
            id: rows[i].get(0),
            name: rows[i].get(1),
            slug: rows[i].get(2),
            description_long: rows[i].get(3),
            description_short: rows[i].get(4),
            image_link: rows[i].get(5),
            thumbnail_link: rows[i].get(6),
            special_offer_image_link: rows[i].get(7),
            video_link: rows[i].get(8),
            gallery: rows[i].get(9),
            tags: rows[i].get(10),
            created: rows[i].get(11),
            edited: rows[i].get(12),
        });
        
        i += 1;
    }

    let return_data: Return<Vec<Continent>> = Return {data};

    serde_json::to_string(&return_data).unwrap()
}
