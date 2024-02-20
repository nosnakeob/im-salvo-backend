use rocket::serde::json::{Json, json};

use auth_macro::loggedin;
use sql_macro::rb_conn;

use crate::domain::req::R;
use crate::domain::user::User;
use crate::framework::jwt::UserClaim;

#[rb_conn]
#[post("/register", data = "<user>")]
pub async fn register(user: Json<User>) -> R {
    match User::select_by_column("username", &user.username).await {
        Ok(users) => {
            if !users.is_empty() {
                return R::fail(Some("username exists"));
            }
        }
        Err(err) => return R::fail(Some(err.to_string())),
    }


    match User::insert(&user).await {
        Ok(data) => println!("{:?}", data),
        Err(err) => return R::fail(Some(err.to_string())),
    };

    R::ok(None)
}

#[rb_conn]
#[post("/login", data = "<login_user>")]
pub async fn login(login_user: Json<User>) -> R {

    match User::select_by_column("username", &login_user.username).await {
        Ok(users) => {
            if users.is_empty() {
                return R::fail(Some("username not exists"));
            }

            let user = users[0].clone();

            if user.password != login_user.password {
                return R::fail(Some("password error"));
            }
            let user_claim = UserClaim {
                id: user.id.unwrap(),
            };

            let token = UserClaim::sign(user_claim);
            println!("{:?}", UserClaim::decode(token.clone()));
            R::ok(Some(json!({ "token": token })))
        }
        Err(err) => return R::fail(Some(err.to_string()))
    }
}


#[loggedin]
#[get("/check")]
pub async fn check() -> R {
    R::ok(None)
}

