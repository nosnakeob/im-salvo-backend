use derive_new::new;
use once_cell::unsync::Lazy;
use rocket::figment::value::Value;
use rocket::Config;
use rocket_jwt::jwt;

use crate::core::utils::config::get_config;

const SECRET_KEY: Lazy<String> = Lazy::new(|| {
    get_config(Config::SECRET_KEY).ok()
        .and_then(Value::into_string)
        .unwrap_or("secret".to_string())
});


// 30day
#[jwt(SECRET_KEY)]
#[derive(new)]
pub struct UserClaim;

