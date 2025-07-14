use rbatis::rbdc::Uuid;
use redis_macros::{FromRedisValue, ToRedisArgs};
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use im_codegen::base_entity;

#[derive(
    Debug, Serialize, Deserialize, Clone, ToSchema, ToRedisArgs, FromRedisValue, PartialEq,
)]
#[serde(rename_all = "snake_case")]
pub enum UserStatus {
    Online,
    Offline,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ConversationType {
    OneOnOne,
    Group,
    AiChat,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MsgType {
    Text,
    Image,
    File,
    System,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ParticipantRole {
    Admin,
    Member,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FriendStatus {
    Pending,
    Accepted,
    Blocked,
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

#[base_entity]
#[derive(Debug, Serialize, Deserialize)]
pub struct Conversation {
    pub id: Option<Uuid>,
    pub r#type: ConversationType,
    pub name: Option<String>,
    pub creator_id: Option<Uuid>,
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

// todo 对象存储
pub struct File;

#[base_entity]
#[derive(Serialize, Deserialize, Debug)]
pub struct ConversationParticipant {
    pub user_id: Uuid,
    pub conversation_id: Uuid,
    pub role: Option<ParticipantRole>,
}

#[base_entity]
#[derive(Serialize, Deserialize, Debug)]
pub struct Friendship {
    pub user_id_1: Uuid,
    pub user_id_2: Uuid,
    pub status: Option<FriendStatus>,
    pub requested_by: Uuid,
}
