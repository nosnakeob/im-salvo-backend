use im_codegen::base_entity;
use rbatis::rbdc::Uuid;
use serde::{Deserialize, Serialize};

pub mod conversation;
pub mod msg;
pub mod user;

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
