use anyhow::{Error, Result};
use redis_macros::{FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, ToRedisArgs, FromRedisValue, PartialEq)]
pub enum ChatMessage {
    UserMessage {
        username: Option<String>,
        content: String,
    },
    UserOnline {
        username: String,
    },
    UserOffline {
        username: String,
    },
    BotMessage {
        content: String,
    },
}

impl ChatMessage {
    pub fn new_user_msg(username: Option<&str>, content: &str) -> Self {
        Self::UserMessage {
            username: username
                .map(str::to_string)
                .or(Some("manager".to_string())),
            content: content.to_string(),
        }
    }

    pub fn new_bot_msg(content: &str) -> Self {
        Self::BotMessage {
            content: content.to_string(),
        }
    }

    pub fn new_user_online(username: &str) -> Self {
        Self::UserOnline {
            username: username.to_string(),
        }
    }

    pub fn new_user_offline(username: &str) -> Self {
        Self::UserOffline {
            username: username.to_string(),
        }
    }

    pub fn to_json_str(&self) -> Result<String> {
        serde_json::to_string(self).map_err(Error::msg)
    }

    pub fn from_json_str(json: &str) -> Result<Self> {
        serde_json::from_str(json).map_err(Error::msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_json_str() {
        let message = ChatMessage::new_user_msg(Some("alice"), "hello world");
        let json_str = message.to_json_str().unwrap();
        assert_eq!(json_str, r#"{"UserMessage":{"username":"alice","content":"hello world"}}"#);

        let message = ChatMessage::new_bot_msg("hello world");
        let json_str = message.to_json_str().unwrap();
        assert_eq!(json_str, r#"{"BotMessage":{"content":"hello world"}}"#);

        let message = ChatMessage::new_user_online("alice");
        let json_str = message.to_json_str().unwrap();
        assert_eq!(json_str, r#"{"UserOnline":{"username":"alice"}}"#);

        let message = ChatMessage::new_user_offline("alice");
        let json_str = message.to_json_str().unwrap();
        assert_eq!(json_str, r#"{"UserOffline":{"username":"alice"}}"#);
    }

    #[test]
    fn test_from_json_str() {
        let json_str = r#"{"UserMessage":{"username":"alice","content":"hello world"}}"#;
        let message = ChatMessage::from_json_str(json_str).unwrap();
        assert_eq!(
            message,
            ChatMessage::UserMessage {
                username: Some("alice".to_string()),
                content: "hello world".to_string()
            }
        );


        let json_str = r#"{"BotMessage":{"content":"hello world"}}"#;
        let message = ChatMessage::from_json_str(json_str).unwrap();
        assert_eq!(
            message,
            ChatMessage::BotMessage {
                content: "hello world".to_string()
            }
        );

        let json_str = r#"{"UserOnline":{"username":"alice"}}"#;
        let message = ChatMessage::from_json_str(json_str).unwrap();
        assert_eq!(
            message,
            ChatMessage::UserOnline {
                username: "alice".to_string()
            }
        );

        let json_str = r#"{"UserOffline":{"username":"alice"}}"#;
        let message = ChatMessage::from_json_str(json_str).unwrap();
        assert_eq!(
            message,
            ChatMessage::UserOffline {
                username: "alice".to_string()
            }
        );
    }
}