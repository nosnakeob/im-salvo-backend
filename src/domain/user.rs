use rbatis::{crud, impl_select, RBatis};
use rocket::request::FromRequest;
use rocket::serde::{Deserialize, Serialize};

use crate::framework::jwt::UserClaim;

#[derive(Debug, Serialize, Deserialize)]
#[derive(Clone)]
pub struct User {
    pub id: Option<u32>,
    pub username: String,
    pub password: String,
}

crud!(User{},"users");
impl_select!(User{select_by_id(id:u32) -> Option => "`where id = #{id} limit 1`"});

impl User {
    pub async fn from_claim(rb: &RBatis, user_claim: &UserClaim) -> Option<Self> {
        User::select_by_id(rb, user_claim.id).await.unwrap()
    }
}
