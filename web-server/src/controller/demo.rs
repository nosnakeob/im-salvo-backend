use std::time::Duration;
use deadpool_redis::Pool;
use redis::{AsyncCommands, ExistenceCheck, Script, SetExpiry, SetOptions};
use rocket::{Config, State};

use web_common::core::constant::cache::token2key;
use crate::domain::user::User;
use web_common::{
    jwt::UserClaim,
    redis::lock::RedisMutex,
    core::resp::R,
};
use web_common::core::AppConfig;

rocket_base_path!("/demo");

#[utoipa::path(context_path = BASE)]
#[get("/redis")]
pub async fn redis_demo(redis_pool: &State<Pool>, m: RedisMutex) -> R {
    // let user = User::default();
    //
    // let claim = UserClaim::new();
    //
    // let token = UserClaim::sign(claim);
    //
    // let mut conn = redis_pool.get().await?;
    //
    // conn.set_ex(token2key(&token), user, 60).await?;
    //
    // let val: Option<User> = conn.get(token2key(&token)).await?;
    // println!("redis get: {:?}", val);
    // let val: bool = conn.exists(token2key(&token)).await?;
    // println!("redis ext: {:?}", val);
    //
    // // 原子设置nx 过期
    // let opts = SetOptions::default()
    //     .conditional_set(ExistenceCheck::NX)
    //     .with_expiration(SetExpiry::EX(1));
    //
    // let mut success: bool = conn.set_options("key", "value", opts).await?;
    // println!("{success}");
    // // tokio::time::sleep(Duration::from_secs(1)).await;
    //
    // success = conn.set_options("key", "value", opts).await?;
    // println!("{success}");

    m.lock("lock1").await?;

    m.unlock("lock1").await?;

    R::no_val_success()
}

// #[transaction]
// #[utoipa::path(context_path = BASE)]
// #[get("/transaction")]
// pub async fn transaction() -> R {
//     let mut user1 = User::select_by_id(1).await?.unwrap();
//     let mut user2 = User::select_by_id(2).await?.unwrap();
//
//     user1.money -= 50;
//     user2.money += 50;
//
//     User::update_by_id(&user1, user1.id.unwrap()).await?;
//
//     // return R::fail("error");
//
//     User::update_by_id(&user2, user2.id.unwrap()).await?;
//
//     R::no_val_success()
// }

#[utoipa::path(context_path = BASE)]
#[get("/config")]
pub async fn config(rocket_config: &Config, app_config: &State<AppConfig>) -> R {
    println!("rocket_config: {:#?}", rocket_config);
    println!("app_config: {:#?}", app_config);

    println!("{}", app_config.database.postgres.url);

    R::no_val_success()
}

