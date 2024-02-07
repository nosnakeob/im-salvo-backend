// use once_cell::sync::Lazy;
// use permit_micro::logged;
// use rocket::Config;
use rocket::http::Status;
use rocket::serde::json::Value;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::{Json, json};
use rocket_jwt::jwt;

// static KEY: Lazy<String> = Lazy::new(|| {
//     let secret = Config::figment().find_value(Config::SECRET_KEY).unwrap();
//     secret.as_str().unwrap().to_owned()
// });
static SECRET_KEY: &str = "secret";

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: Option<String>,
    username: String,
    password: String,
}

#[jwt(SECRET_KEY, exp = 120)]
pub struct UserClaim {
    id: String,
}


#[post("/login", data = "<user>")]
pub async fn login(mut user: Json<User>) -> Value {
    println!("{}", SECRET_KEY);
    println!("{:?}", user);

    // 存数据库更新id
    if user.id == None {
        user.id = Some("1".to_string());
    }

    let user_claim = UserClaim {
        id: user.id.clone().unwrap(),
    };

    let token = UserClaim::sign(user_claim);
    // println!("{:#?}", UserClaim::decode(token.clone()));
    json!({
        "code": Status::Ok.code,
        "token": token
    })
}

// #[logged]
#[get("/check")]
pub async fn check(_user_claim: UserClaim) -> Value {
    json!({
        "code": Status::Ok.code,
        "msg": "success"
    })
}


