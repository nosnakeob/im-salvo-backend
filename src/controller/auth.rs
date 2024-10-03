use web_common::jwt::UserClaim;
use serde_json::json;
use deadpool_redis::Pool;
use redis::AsyncCommands;
use rocket::serde::json::Json;
use rocket::State;

use web_common::{
    bail,
    core::{
        resp::R,
        constant::cache::token2key,
        utils,
    },
};
use crate::domain::user::User;

rocket_base_path!("/auth");

#[rbatis_conn]
#[utoipa::path(context_path = BASE)]
#[post("/register", data = "<register_user>")]
pub async fn register(mut register_user: Json<User>) -> R {
    let user = User::select_by_name(&register_user.username).await?;

    if user.is_some() {
        bail!("username exists");
    }

    register_user.password = utils::password::encode(&register_user.password);

    User::insert(&register_user).await?;

    R::no_val_success()
}


#[rbatis_conn]
#[utoipa::path(context_path = BASE)]
#[post("/login", data = "<login_user>")]
pub async fn login(login_user: Json<User>, redis_pool: &State<Pool>) -> R {
    let user = User::select_by_name(&login_user.username).await?;

    let user = match user {
        Some(user) => user,
        None => bail!("user not exists")
    };

    if !utils::password::verify(&user.password, &login_user.password) {
        bail!("password error");
    }

    let user_claim = UserClaim::new();

    let token = UserClaim::sign(user_claim);

    redis_pool.get().await?.set_ex(token2key(&token), user, 3600).await?;

    R::success(json!({ "token": token }))
}

#[utoipa::path(context_path = BASE)]
#[get("/check")]
pub async fn check(user: User) -> R {
    R::success(json!({ "user": user }))
}
