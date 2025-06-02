use redis_macros::{FromRedisValue, ToRedisArgs};
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema, ToRedisArgs, FromRedisValue)]
pub struct User {
    pub id: Option<u32>,
    pub username: String,
    pub password: String,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: Some(1),
            username: "snake".to_string(),
            password: "123123".to_string(),
        }
    }
}
