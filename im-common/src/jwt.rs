use serde::{Deserialize, Serialize};
use std::cell::LazyCell;
use time::{Duration, OffsetDateTime};

// const SECRET_KEY: LazyCell<String> = LazyCell::new(|| {
//     get_config(Config::SECRET_KEY).ok()
//         .and_then(Value::into_string)
//         .unwrap_or("secret".to_string())
// });
pub const SECRET_KEY: LazyCell<String> = LazyCell::new(|| "secret".to_string());

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub username: String,
    pub exp: i64,
}

impl JwtClaims {
    pub fn new(username: &str, duration: Duration) -> Self {
        JwtClaims {
            username: username.to_string(),
            exp: (OffsetDateTime::now_utc() + duration).unix_timestamp(),
        }
    }
}
