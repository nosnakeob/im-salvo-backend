use rocket::serde::json::{Json, json};

use crate::common::utils;
use crate::domain::resp::R;
use crate::domain::user::User;
use crate::framework::jwt::UserClaim;

rocket_base_path!("/auth");

#[rb_conn]
#[utoipa::path]
#[post("/register", data = "<user>")]
pub async fn register(mut user: Json<User>) -> R {
    match User::select_by_column("username", &user.username).await {
        Ok(users) => {
            if !users.is_empty() {
                return R::fail("username exists");
            }
        }
        Err(err) => return R::fail(err.to_string()),
    }

    user.password = utils::password::encode(&user.password);

    match User::insert(&user).await {
        Ok(data) => println!("{:?}", data),
        Err(err) => return R::fail(err.to_string()),
    };

    R::ok(None)
}

#[rb_conn]
#[utoipa::path]
#[post("/login", data = "<login_user>")]
pub async fn login(login_user: Json<User>) -> R {
    match User::select_by_column("username", &login_user.username).await {
        Ok(users) => {
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
            R::ok(Some(json!({ "token": token })))
        }
        Err(err) => return R::fail(err.to_string())
    }
}


#[loggedin]
#[utoipa::path]
#[get("/check")]
pub async fn check() -> R {
    R::ok(None)
}
