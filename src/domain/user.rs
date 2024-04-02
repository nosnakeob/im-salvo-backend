use redis::AsyncCommands;
use redis_macros::{FromRedisValue, ToRedisArgs};
use rocket::http::Status;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use rocket::serde::{Deserialize, Serialize};
use rocket_db_pools::Connection;
use utoipa::ToSchema;

use crate::common::constant::cache::token2key;
use crate::framework::redis::RedisCache;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, ToRedisArgs, FromRedisValue)]
pub struct User {
    pub id: Option<u32>,
    pub username: String,
    pub password: String,
}


impl Default for User {
    fn default() -> Self {
        Self {
            id: Some(123332),
            username: "snake".to_string(),
            password: "123123".to_string(),
        }
    }
}


#[async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth = req.headers().get_one("Authorization").unwrap();

        let token = auth.strip_prefix("Bearer ").unwrap();

        let mut redis_cache: Connection<RedisCache> = req.guard().await.unwrap();

        if let Ok(user) = redis_cache.get(token2key(token)).await {
            return Outcome::Success(user);
        };

        Outcome::Error((Status::Unauthorized, ()))
    }
}