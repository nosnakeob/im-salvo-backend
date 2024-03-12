use once_cell::unsync::Lazy;
use rocket::Config;
use rocket_jwt::jwt;

use crate::common::utils::config::get_config;

const SECRET_KEY: Lazy<String> = Lazy::new(|| {
    get_config(Config::SECRET_KEY).unwrap().as_str().unwrap().into()
});


// 2h
#[jwt(SECRET_KEY, exp = 7200)]
pub struct UserClaim {
    pub id: u32,
}


