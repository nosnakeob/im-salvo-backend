use rocket::serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct User {
    pub id: Option<u32>,
    pub username: String,
    pub password: String,
}

