#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate tracing;

use crate::controller::*;
use crate::middleware::jwt::check_auth;
use crate::middleware::rbatis::set_db;
use crate::middleware::redis::set_redis;
use salvo::cors::Cors;
use salvo::http::Method;
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
pub async fn build_salvo() -> Service {
    let jwt: JwtAuth<JwtClaims, _> = JwtAuth::new(ConstDecoder::from_secret(SECRET_KEY.as_bytes()))
        .finders(vec![Box::new(HeaderFinder::new())])
        .force_passed(true);

    let cors = Cors::new()
        .allow_origin("*")
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_headers("authorization")
        .into_handler();

    let router = Router::new()
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

    Service::new(router)
        .hoop(set_db)
        .hoop(set_redis)
        .hoop(jwt)
        .hoop(cors)
}
