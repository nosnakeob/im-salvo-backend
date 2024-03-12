use rocket::serde::json::{Json, json};

use crate::common::utils;
use crate::domain::user::User;
use crate::framework::jwt::UserClaim;
use crate::framework::rocket::resp::R;

rocket_base_path!("/auth");

#[rb_conn]
#[utoipa::path(context_path = "/auth")]
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
#[utoipa::path(context_path = "/auth")]
#[post("/login", data = "<login_user>")]
pub async fn login(login_user: Json<User>) -> R {
    let users = User::select_by_column("username", &login_user.username).await?;

    if users.is_empty() {
        return R::fail("username not exists");
    }

    let user = users[0].clone();

    if !utils::password::verify(&user.password, &login_user.password) {
        return R::fail("password error");
    }
    let user_claim = UserClaim {
        id: user.id.unwrap(),
    };

    let token = UserClaim::sign(user_claim);
    R::success(json!({ "token": token }))
}


#[loggedin]
#[utoipa::path(context_path = "/auth")]
#[get("/check")]
pub async fn check() -> R {
    R::no_val_success()
}
