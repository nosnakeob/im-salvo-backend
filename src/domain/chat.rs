use derive_new::new;
use redis_macros::{FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, ToRedisArgs, FromRedisValue)]
pub struct ChatMessage {
    pub username: Option<String>,
    pub content: String,
}

impl ChatMessage {
    pub fn new(username: Option<String>, content: String) -> Self {
        Self {
            username: username.or(Some("manager".to_string())),
            content,
        }
    }
}