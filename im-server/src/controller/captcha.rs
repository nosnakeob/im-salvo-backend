use deadpool_redis::Pool;
use serde_json::json;
use common::core::resp::R;
use redis::AsyncCommands;
use rocket::State;

rocket_base_path!("/captcha");

#[utoipa::path(context_path = BASE)]
#[get("/code")]
pub async fn code(redis_pool: &State<Pool>) -> R {
    let code = "1234";

    let _: bool = redis_pool.get().await?.set_ex("captcha", code, 60).await.unwrap();

    R::success(json!({ "code": code }))
}