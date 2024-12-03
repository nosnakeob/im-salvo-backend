use derive_new::new;
use rocket::figment::value::Value;
use rocket::Config;
use rocket_jwt::jwt;
use std::cell::LazyCell;

use crate::core::utils::config::get_config;

const SECRET_KEY: LazyCell<String> = LazyCell::new(|| {
    get_config(Config::SECRET_KEY).ok()
        .and_then(Value::into_string)
        .unwrap_or("secret".to_string())
});


// 30day
#[jwt(SECRET_KEY)]
#[derive(new)]
pub struct UserClaim;

