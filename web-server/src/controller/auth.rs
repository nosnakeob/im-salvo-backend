use crate::domain::user::User;
use crate::ApiResponse;
use api_response::prelude::*;
use jsonwebtoken::EncodingKey;
use rbatis::RBatis;
use salvo::oapi::extract::JsonBody;
use salvo::prelude::*;
use serde_json::{json, Value};
use time::Duration;
use web_common::jwt::{JwtClaims, SECRET_KEY};
use web_common::utils;

#[endpoint]
pub async fn register(json: JsonBody<User>, depot: &mut Depot) -> ApiResponse<()> {
    let rb = depot.obtain_mut::<RBatis>().unwrap();

    let mut register_user = json.into_inner();

    let user = User::select_by_name(rb, &register_user.username)
        .await
        .unwrap();

    if user.is_some() {
        // bail!("username exists");
        return ApiResponse::from_error_msg(
            StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            "username exists",
        );
    }

    register_user.password = utils::password::encode(&register_user.password);

    User::insert(rb, &register_user).await.unwrap();

    ().api_response_without_meta()
}

#[endpoint]
pub async fn login(json: JsonBody<User>, depot: &Depot) -> ApiResponse<Value> {
    // let pool = depot.obtain::<Pool>().unwrap();
    let rb = depot.obtain::<RBatis>().unwrap();

    let login_user = json.into_inner();

    let user = User::select_by_name(rb, &login_user.username)
        .await
        .unwrap();

    let user = match user {
        Some(user) => user,
        None => {
            // todo bail!
            return ApiResponse::from_error_msg(
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                "user not exists",
            );
        }
    };

    if !utils::password::verify(&user.password, &login_user.password) {
        return ApiResponse::from_error_msg(
            StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            "password error",
        );
    }

    let claim = JwtClaims::new(&user.username, Duration::hours(6));

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claim,
        &EncodingKey::from_secret(SECRET_KEY.as_bytes()),
    )
    .unwrap();

    // token -> user 登录鉴权
    // let _: () = pool
    //     .get()
    //     .await
    //     .unwrap()
    //     // .set_ex(token2key(&token), user, 3600)
    //     .set_ex(&token, user, 3600)
    //     .await
    //     .unwrap();

    json!({ "token": token }).api_response_without_meta()
}

#[endpoint]
pub async fn check() -> ApiResponse<()> {
    ().api_response_without_meta()
}
