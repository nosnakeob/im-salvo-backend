use rbatis::{crud, impl_select};
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[derive(Clone)]
pub struct User {
    pub id: Option<u32>,
    pub username: String,
    pub password: String,
}

crud!(User{},"users");
impl_select!(User{select_by_id(id:u32) -> Option => "`where id = #{id} limit 1`"});
