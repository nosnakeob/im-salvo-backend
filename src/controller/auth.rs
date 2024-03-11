use rocket::serde::json::{Json, json};

use crate::common::utils;
use crate::domain::user::User;
use crate::framework::jwt::UserClaim;
use crate::framework::rocket::resp::R;

rocket_base_path!("/auth");

#[rb_conn]
#[utoipa::path]
#[post("/register", data = "<user>")]
pub async fn register(mut user: Json<User>) -> R {
    let users = User::select_by_column("username", &user.username).await?;

    if !users.is_empty() {
        return R::Fail("username exists".into());
    }

    user.password = utils::password::encode(&user.password);

    let data = User::insert(&user).await?;

    println!("{:?}", data);

    R::Success(None::<u8>.into())
}

#[rb_conn]
#[utoipa::path]
#[post("/login", data = "<login_user>")]
pub async fn login(login_user: Json<User>) -> R {
    let users = User::select_by_column("username", &login_user.username).await?;

    if users.is_empty() {
        return R::Fail("username not exists".into());
    }

    let user = users[0].clone();

    if !utils::password::verify(&user.password, &login_user.password) {
        return R::Fail("password error".into());
    }
    let user_claim = UserClaim {
        id: user.id.unwrap(),
    };

    let token = UserClaim::sign(user_claim);
    R::Success(Some(json!({ "token": token })).into())
}


#[loggedin]
#[utoipa::path]
#[get("/check")]
pub async fn check() -> R {
    R::Success(None::<u8>.into())
}
