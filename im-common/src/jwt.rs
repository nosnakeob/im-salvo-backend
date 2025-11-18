use rbatis::rbdc::Uuid;
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
