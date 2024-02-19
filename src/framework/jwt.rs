use once_cell::sync::Lazy;
use rocket::Config;
use rocket_jwt::jwt;

use crate::common::utils::config::get_config;

static SECRET_KEY: Lazy<String> = Lazy::new(|| {
    let secret = get_config(Config::SECRET_KEY).unwrap();
    secret.as_str().unwrap().to_owned()
});


// 2h
#[jwt(SECRET_KEY, exp = 7200)]
pub struct UserClaim {
    pub id: u32,
}


