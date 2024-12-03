use deadpool_redis::Pool;
use redis::AsyncCommands;
use redis_macros::{FromRedisValue, ToRedisArgs};

use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::serde::{Deserialize, Serialize};
use rocket::{Request, State};
use utoipa::ToSchema;

use web_common::core::constant::cache::token2key;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, ToRedisArgs, FromRedisValue)]
pub struct User {
    pub id: Option<u32>,
    pub username: String,
    pub password: String,
}


impl Default for User {
    fn default() -> Self {
        Self {
            id: Some(1),
            username: "snake".to_string(),
            password: "123123".to_string(),
        }
    }
}

fn extract_token(req: &Request) -> Option<String> {
    // header Authorization: Bearer token
    req.headers().get_one("Authorization")
        .and_then(|auth| auth.strip_prefix("Bearer "))
        .map(|token| token.to_string())
        // query ?Authorization=Bearer token
        .or(req.uri().query().and_then(|query|
            query.split('&')
                .filter_map(|s| s.strip_prefix("Authorization=Bearer%20"))
                .map(|s| s.to_string())
                .last()
        ))
}

#[async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let redis_pool: &State<Pool> = req.guard().await.unwrap();
        let mut conn = redis_pool.get().await.unwrap();

        if let Some(token) = extract_token(req) {
            if let Ok(user) = conn.get(token2key(&token)).await {
                return Outcome::Success(user);
            };
        }

        Outcome::Error((Status::Unauthorized, ()))
    }
}