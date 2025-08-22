use rbatis::rbdc::Uuid;
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
    pub id: Uuid,
    pub exp: i64,
}

impl JwtClaims {
    pub fn new(id: Uuid, duration: Duration) -> Self {
        JwtClaims {
            id,
            exp: (OffsetDateTime::now_utc() + duration).unix_timestamp(),
        }
    }
}
