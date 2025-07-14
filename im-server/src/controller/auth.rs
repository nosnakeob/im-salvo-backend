use crate::ApiResponse;
use crate::domain::db::User;
use api_response::prelude::*;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::EncodingKey;
use rbatis::RBatis;
use salvo::oapi::extract::JsonBody;
use salvo::prelude::*;
use serde_json::{Value, json};
use time::Duration;
use im_codegen::bail;
use im_common::jwt::{JwtClaims, SECRET_KEY};

#[endpoint]
pub async fn register(json: JsonBody<User>, depot: &mut Depot) -> ApiResponse<()> {
    let rb = depot.obtain_mut::<RBatis>().unwrap();

    let mut register_user = json.into_inner();

    if User::select_by_name(rb, &register_user.username)
        .await
        .unwrap()
        .is_some()
    {
        bail!("username exists");
    }

    register_user.password = hash(register_user.password, DEFAULT_COST).unwrap();

    User::insert(rb, &register_user).await.unwrap();

    ().api_response_without_meta()
}

#[endpoint]
pub async fn login(json: JsonBody<User>, depot: &Depot) -> ApiResponse<Value> {
    // let pool = depot.obtain::<Pool>().unwrap();
    let rb = depot.obtain::<RBatis>().unwrap();

    let login_user = json.into_inner();

    let user = match User::select_by_name(rb, &login_user.username).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            bail!("user not exists");
        }
        Err(e) => {
            bail!(e.to_string());
        }
    };

    if !verify(login_user.password, &user.password).unwrap() {
        bail!("password error");
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
