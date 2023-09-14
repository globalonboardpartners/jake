use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Auth {
    pub id: Option<i32>,
    pub email: Option<String>,
    pub username: String,
    pub password: String,
    pub salt: Option<String>,
    pub api_key: Option<Uuid>,
    pub security_level: Option<i16>,
    pub employee_id: Option<i32>,
    pub status: Option<Status>,
    pub last_login: Option<String>,
    pub failed_login_attempts: Option<i32>,
    pub created: Option<String>,
    pub edited: Option<String>,
}

#[derive(sqlx::Type, Serialize, Deserialize, Debug)]
#[sqlx(type_name = "status")]
pub enum Status {
    Active,
    Inactive,
    Suspended
}
