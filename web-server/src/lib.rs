#[macro_use]
extern crate rbatis;

use crate::controller::auth::*;
use crate::controller::index;
use crate::middleware::jwt::auth;
use crate::middleware::rbatis::set_db;
use crate::middleware::redis::set_redis_pool;
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

    Router::new()
        .get(index)
        .hoop(set_db)
        .hoop(set_redis_pool)
        .hoop(jwt)
        .push(
            Router::with_path("auth")
                .push(Router::with_path("register").post(register))
                .push(Router::with_path("login").post(login)),
        )
        .push(
            Router::new()
                .hoop(auth)
                .push(Router::with_path("auth").push(Router::with_path("check").get(check))),
        )
}

// 创建CORS中间件
// fn cors_middleware() -> impl Handler {
//     cors::Cors::new()
//         .allow_origin("*")
//         .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
//         .allow_headers("*")
//         .into_handler()
// }
