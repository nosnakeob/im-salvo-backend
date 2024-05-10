use deadpool_redis::Pool;
use redis::AsyncCommands;
use redis_macros::{FromRedisValue, ToRedisArgs};

use rocket::http::Status;
use rocket::{Request, State};
use rocket::request::{FromRequest, Outcome};
use rocket::serde::{Deserialize, Serialize};
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

        let redis_pool: &State<Pool> = req.guard().await.unwrap();

        if let Ok(user) = redis_pool.get().await.unwrap().get(token2key(token)).await {
            return Outcome::Success(user);
        };

        Outcome::Error((Status::Unauthorized, ()))
    }
}