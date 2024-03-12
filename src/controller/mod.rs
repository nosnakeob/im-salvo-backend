use std::time::Duration;

use rocket::tokio::time::sleep;
use rocket_db_pools::Connection;
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;

use crate::framework::redis::RedisCache;
use crate::framework::rocket::resp::R;

pub mod auth;
pub mod chat;

rocket_base_path!("/");

#[utoipa::path]
#[get("/")]
pub async fn index(mut redis_cache: Connection<RedisCache>) -> R {
    redis_cache.set_ex("key", "value", 1).await?;

    sleep(Duration::from_secs(2)).await;

    let val: String = redis_cache.get("key").await?;
    println!("redis get: {:?}", val);


    R::no_val_success()
}


#[rb_conn]
#[utoipa::path]
#[get("/pool")]
pub async fn pool() -> R {
    R::success(rb.get_pool().unwrap().state().await.as_map().unwrap())
}