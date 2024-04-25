use rocket_db_pools::Connection;
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;

use crate::common::constant::cache::token2key;
use crate::domain::user::User;
use crate::framework::jwt::UserClaim;
use crate::framework::redis::RedisCache;
use crate::framework::rocket::resp::R;

rocket_base_path!("/demo");
#[utoipa::path(context_path = BASE)]
#[get("/redis")]
pub async fn redis_demo(mut redis_cache: Connection<RedisCache>) -> R {
    let user = User::default();

    let claim = UserClaim::new();

    let token = UserClaim::sign(claim);

    redis_cache.set_ex(token2key(&token), user, 60).await?;

    // sleep(Duration::from_secs(2)).await;

    // let val: String = redis_cache.get("key").await?;
    let val: Option<User> = redis_cache.get(token2key(&token)).await?;
    println!("redis get: {:?}", val);
    let val: bool = redis_cache.exists(token2key(&token)).await?;
    println!("redis ext: {:?}", val);


    R::no_val_success()
}

#[transaction]
#[utoipa::path(context_path = BASE)]
#[get("/transaction")]
pub async fn transaction() -> R {
    let mut user1 = User::select_by_id(1).await?.unwrap();
    let mut user2 = User::select_by_id(2).await?.unwrap();

    user1.money -= 50;
    user2.money += 50;

    User::update_by_id(&user1, user1.id.unwrap()).await?;

    // return R::fail("error");

    User::update_by_id(&user2, user2.id.unwrap()).await?;

    R::no_val_success()
}
