use rocket::serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct User {
    pub id: Option<u32>,
    pub username: String,
    pub password: String,
}

crud!(User{},"users");
impl_select!(User{select_by_id(id:u32) -> Option => "`where id = #{id} limit 1`"});
