use deadpool_redis::Config;
use redis::AsyncCommands;
use rocket::fairing::AdHoc;
use rocket::request::FromRequest;

use crate::core::utils::config::get_config;

pub mod lock;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("init redis pool", |rocket| async {
        let url = get_config("database.redis.url").unwrap().into_string().unwrap();
        let pool = Config::from_url(url.clone()).create_pool(None).unwrap();
        // 用于发布订阅
        let client = redis::Client::open(url).unwrap();

        rocket.manage(pool)
            .manage(client)
    })
}
