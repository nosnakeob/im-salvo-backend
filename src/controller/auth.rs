use rocket::serde::json::{Json, json};
use rocket_db_pools::Connection;
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;

use crate::common::constant::cache::token2key;
use crate::common::utils;
use crate::domain::user::User;
use crate::framework::jwt::UserClaim;
use crate::framework::redis::RedisCache;
use crate::framework::rocket::resp::R;

rocket_base_path!("/auth");

#[rb_conn]
#[utoipa::path(context_path = BASE)]
#[post("/register", data = "<user>")]
pub async fn register(mut user: Json<User>) -> R {
    let users = User::select_by_column("username", &user.username).await?;

    if !users.is_empty() {
        return R::fail("username exists");
    }

    user.password = utils::password::encode(&user.password);

    let data = User::insert(&user).await?;

    println!("{:?}", data);

    R::no_val_success()
}

#[rb_conn]
#[utoipa::path(context_path = BASE)]
#[post("/login", data = "<login_user>")]
pub async fn login(login_user: Json<User>, mut redis_cache: Connection<RedisCache>) -> R {
    let users = User::select_by_column("username", &login_user.username).await?;

    if users.is_empty() {
        return R::fail("username not exists");
    }

    let user = &users[0];

    if !utils::password::verify(&user.password, &login_user.password) {
        return R::fail("password error");
    }
    let user_claim = UserClaim::new();

    let token = UserClaim::sign(user_claim);

    redis_cache.set_ex(token2key(&token), user, 3600).await?;

    R::success(json!({ "token": token }))
}


#[utoipa::path(context_path = BASE)]
#[get("/check")]
pub async fn check(user: User) -> R {
    R::no_val_success()
}
