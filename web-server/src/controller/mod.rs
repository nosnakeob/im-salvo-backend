use api_response::ApiResponse;
use salvo::prelude::*;

pub mod auth;
// pub mod chat;
// pub mod captcha;
// pub mod demo;

// rocket_base_path!("/");

#[endpoint]
pub async fn index() -> ApiResponse<&'static str, ()> {
    ApiResponse::from_success("Hello, world!")
}
