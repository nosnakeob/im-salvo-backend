use im_codegen::base_entity;
use rbatis::rbdc::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ConversationType {
    OneOnOne,
    Group,
    AiChat,
}

#[base_entity]
#[derive(Debug, Serialize, Deserialize)]
pub struct Conversation {
    pub id: Option<Uuid>,
    pub r#type: ConversationType,
    pub name: Option<String>,
    pub creator_id: Option<Uuid>,
}
