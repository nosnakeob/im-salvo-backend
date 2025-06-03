#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate tracing;

use crate::controller::*;
use crate::middleware::jwt::check_auth;
use crate::middleware::rbatis::set_db;
use crate::middleware::redis::set_redis;
use salvo::jwt_auth::{ConstDecoder, HeaderFinder};
use salvo::prelude::*;
use web_common::jwt::{JwtClaims, SECRET_KEY};

pub mod controller;
pub mod domain;
pub mod mapper;
pub mod middleware;

// #[cfg(test)]
// pub mod test;

/// 构建Salvo应用程序
pub async fn build_salvo() -> Router {
    let jwt: JwtAuth<JwtClaims, _> = JwtAuth::new(ConstDecoder::from_secret(SECRET_KEY.as_bytes()))
        .finders(vec![Box::new(HeaderFinder::new())])
        .force_passed(true);

    let router = Router::new()
        .hoop(set_db)
        .hoop(set_redis)
        .hoop(jwt)
        .get(index)
        .push(
            Router::with_path("auth")
                .push(Router::with_path("register").post(register))
                .push(Router::with_path("login").post(login)),
        )
        .push(
            Router::new()
                .hoop(check_auth)
                .push(Router::with_path("auth").push(Router::with_path("check").get(check))),
        )
        .push(
            Router::with_path("chat")
                .get(user_connected)
                .push(Router::with_path("{id}").post(chat_send)), // 发送消息
        );

    let doc = OpenApi::new("test api", "0.0.1").merge_router(&router);

    let router = router
        .unshift(doc.into_router("/api-doc/openapi.json"))
        .unshift(SwaggerUi::new("/api-doc/openapi.json").into_router("/swagger-ui"));

    router
}

// 创建CORS中间件
// fn cors_middleware() -> impl Handler {
//     cors::Cors::new()
//         .allow_origin("*")
//         .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
//         .allow_headers("*")
//         .into_handler()
// }
