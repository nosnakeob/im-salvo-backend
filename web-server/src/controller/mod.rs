use api_response::ApiResponse;
use salvo::prelude::*;

mod auth;
mod chat;

#[endpoint]
pub async fn index() -> ApiResponse<&'static str, ()> {
    ApiResponse::from_success("Hello, world!")
}

// 暴露接口
pub use self::{
    auth::{check, login, register},
    chat::{chat_send, user_connected},
};
