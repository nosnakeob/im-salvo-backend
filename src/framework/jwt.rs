use derive_new::new;
use once_cell::unsync::Lazy;
use rocket::Config;
use rocket_jwt::jwt;

use crate::common::utils::config::get_config;

const SECRET_KEY: Lazy<String> = Lazy::new(|| {
    get_config(Config::SECRET_KEY).unwrap().as_str().unwrap().into()
});


// 30day
#[jwt(SECRET_KEY)]
#[derive(new)]
pub struct UserClaim;



