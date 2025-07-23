use crate::ApiResponse;
use crate::hoops::auth_hoop;
use api_response::prelude::*;
use salvo::prelude::*;

mod auth;
mod chat;

#[endpoint]
pub async fn index() -> ApiResponse<&'static str> {
    "Hello, world!".api_response_without_meta()
}

/// 构建路由
pub fn root() -> Router {
    let router = Router::new()
        .get(index)
        .push(
            Router::with_path("auth")
                .push(Router::with_path("register").post(auth::register))
                .push(Router::with_path("login").post(auth::login)),
        )
        .push(
            Router::new()
                .hoop(auth_hoop())
                .push(Router::with_path("auth").push(Router::with_path("check").get(auth::check))),
        )
        .push(
            Router::with_path("chat")
                .get(chat::user_connected)
                .push(Router::with_path("{id}").post(chat::chat_send)), // 发送消息
        );

    let doc = OpenApi::new("test api", "0.0.1").merge_router(&router);

    router
        .unshift(doc.into_router("/api-doc/openapi.json"))
        .unshift(SwaggerUi::new("/api-doc/openapi.json").into_router("/swagger-ui"))
}
