use im_codegen::base_entity;
use rbatis::rbdc::Uuid;
use redis_macros::{FromRedisValue, ToRedisArgs};
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Serialize, Deserialize, Clone, ToSchema, ToRedisArgs, FromRedisValue, PartialEq,
)]
#[serde(rename_all = "snake_case")]
pub enum UserStatus {
    Online,
    Offline,
}

#[base_entity]
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema, ToRedisArgs, FromRedisValue)]
pub struct User {
    pub id: Option<Uuid>,
    pub username: String,
    pub password: String,
    pub status: Option<UserStatus>,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: None,
            username: "alice".to_string(),
            password: "alice123".to_string(),
            status: None,
            created_at: None,
            updated_at: None,
        }
    }
}
crud!(User {}, "users");
impl_select!(User{select_by_id(id:u32) -> Option => "`where id = #{id} limit 1`"}, "users");
impl_select!(User{select_by_name(name:&str) -> Option => "`where username = #{name} limit 1`"}, "users");
impl_update!(User{update_by_id(id:u32) => "`where id = #{id}`"}, "users");
impl_delete!(User{delete_by_name(name:&str) => "`where username = #{name}`"}, "users");
