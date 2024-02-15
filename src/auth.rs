use once_cell::sync::Lazy;
use permit_micro::loggedin;
use rbatis::{crud, RBatis};
use rocket::{Config, State};
use rocket::http::Status;
use rocket::serde::json::Value;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::{Json, json};
use rocket_jwt::jwt;

static SECRET_KEY: Lazy<String> = Lazy::new(|| {
    let secret = Config::figment().find_value(Config::SECRET_KEY).unwrap();
    secret.as_str().unwrap().to_owned()
});


#[derive(Debug, Serialize, Deserialize)]
#[derive(Clone)]
pub struct User {
    id: Option<u32>,
    username: String,
    password: String,
}

crud!(User{},"users");


#[jwt(SECRET_KEY, exp = 120)]
pub struct UserClaim {
    id: u32,
}

#[post("/register", data = "<user>")]
pub async fn register(user: Json<User>, rb: &State<RBatis>) -> (Status, String) {
    println!("{:?}", user);
    let rb = &**rb;

    match User::select_by_column(rb, "username", &user.username).await {
        Ok(users) => {
            if !users.is_empty() {
                return (Status::BadRequest, "username already exists".to_string());
            }
        }
        Err(err) => return (Status::BadRequest, err.to_string()),
    }


    match User::insert(rb, &user).await {
        Ok(data) => println!("{:?}", data),
        Err(err) => {
            return (Status::BadRequest, err.to_string());
        }
    };

    println!("{:?}", user);

    (Status::Ok, "success".to_string())
}

#[post("/login", data = "<login_user>")]
pub async fn login(login_user: Json<User>, rb: &State<RBatis>) -> Value {
    let rb = &**rb;

    match User::select_by_column(rb, "username", &login_user.username).await {
        Ok(users) => {
            if users.is_empty() {
                return json!({
                    "code": Status::BadRequest,
                    "msg": "username not exists"
                });
            }

            let user = users[0].clone();

            if user.password != login_user.password {
                return json!({
                    "code": Status::BadRequest,
                    "msg": "password error"
                });
            }
            let user_claim = UserClaim {
                id: user.id.unwrap(),
            };
            let token = UserClaim::sign(user_claim);
            json!({
                "code": Status::Ok,
                "token": token
            })
        }
        Err(err) => json!({
                "code": Status::BadRequest,
                "msg": err.to_string()
            })
    }
}

#[loggedin]
#[get("/check")]
pub async fn check() -> Value {
    json!({
        "code": Status::Ok,
        "msg": "success"
    })
}


