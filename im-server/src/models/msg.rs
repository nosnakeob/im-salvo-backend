use im_codegen::base_entity;
use rbatis::rbdc::Uuid;
use redis_macros::{FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MsgType {
    Text,
    Image,
    File,
    System,
}

#[base_entity]
#[derive(Serialize, Deserialize, Debug, ToRedisArgs, FromRedisValue, PartialEq)]
pub struct Message {
    id: Option<i64>,
    conversation_id: Uuid,
    sender_id: Uuid,
    r#type: MsgType,
    content: Value,
}
crud!(Message {});
