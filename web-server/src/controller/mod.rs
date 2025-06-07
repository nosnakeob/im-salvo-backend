use crate::ApiResponse;
use api_response::prelude::*;
use salvo::prelude::*;

mod auth;
mod chat;

#[endpoint]
pub async fn index() -> ApiResponse<&'static str> {
    "Hello, world!".api_response_without_meta()
}

// 暴露接口
pub use self::{
    auth::{check, login, register},
    chat::{chat_send, user_connected},
};
