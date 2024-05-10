use deadpool_redis::Config;
use rocket::fairing::AdHoc;

use crate::core::utils::config::get_config;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("init redis pool", |rocket| async {
        let url = get_config("database.redis.url").unwrap().into_string().unwrap();
        let pool = Config::from_url(url).create_pool(None).unwrap();

        rocket.manage(pool)
    })
}

