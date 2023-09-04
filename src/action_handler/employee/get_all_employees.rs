use crate::db::QueryBuilder;
use crate::data_types::structs::{Employee, Return};

pub async fn execute() -> String {
    let mut data = vec![];

    let rows = QueryBuilder::get_all("employee").await;
    
    let mut i: usize = 0;
    loop {
        if i > rows.len() - 1 {
            break;
        }

        data.push(Employee {
            id: rows[i].get(0),
            name: rows[i].get(1),
            position: rows[i].get(2),
            bio: rows[i].get(3),
            image_url: rows[i].get(4),
        });
        
        i += 1;
    }

    let return_data: Return<Vec<Employee>> = Return {data};

    serde_json::to_string(&return_data).unwrap()
}
