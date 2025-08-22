use im_codegen::base_entity;
use rbatis::rbdc::Uuid;
use redis_macros::{FromRedisValue, ToRedisArgs};
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema, ToRedisArgs, FromRedisValue)]
#[serde(rename_all = "snake_case")]
pub enum MsgType {
    Text,
    Image,
    File,
    System,
}

#[base_entity]
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema, ToRedisArgs, FromRedisValue)]
pub struct Message {
    pub id: Option<Uuid>,
    pub conversation_id: Uuid,
    // 可以从jwt获取
    pub sender_id: Option<Uuid>,
    pub r#type: MsgType,
    pub content: Value,
}
crud!(Message {});

impl Default for Message {
    fn default() -> Self {
        Self {
            id: None,
            conversation_id: Uuid::default(),
            sender_id: None,
            r#type: MsgType::Text,
            content: Value::from("hahahi"),
            created_at: None,
            updated_at: None,
        }
    }
}
