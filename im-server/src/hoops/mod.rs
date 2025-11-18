use im_common::config::CONFIG;
use im_common::jwt::JwtClaims;
use salvo::cors::{AllowHeaders, AllowMethods, AllowOrigin, Cors, CorsHandler};
use salvo::jwt_auth::{ConstDecoder, HeaderFinder};
use salvo::prelude::*;

/// jwt校验
pub fn auth_hoop() -> JwtAuth<JwtClaims, ConstDecoder> {
    JwtAuth::new(ConstDecoder::from_secret(CONFIG.jwt.secret.as_bytes()))
        .finders(vec![Box::new(HeaderFinder::new())])
}

pub fn cors_hoop() -> CorsHandler {
    Cors::new()
        .allow_origin(AllowOrigin::any())
        .allow_methods(AllowMethods::any())
        .allow_headers(AllowHeaders::any())
        .into_handler()
}
