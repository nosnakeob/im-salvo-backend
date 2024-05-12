use anyhow::{anyhow, bail, Result};
use deadpool_redis::{Config, Pool};
use redis::{AsyncCommands, ExistenceCheck, SetExpiry, SetOptions};
use rocket::{Request, State};
use rocket::fairing::AdHoc;
use rocket::request::{FromRequest, Outcome};
use rocket::http::Status;

use crate::core::utils::config::get_config;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("init redis pool", |rocket| async {
        let url = get_config("database.redis.url").unwrap().into_string().unwrap();
        let pool = Config::from_url(url).create_pool(None).unwrap();

        rocket.manage(pool)
    })
}

// 异步锁
pub struct RedisMutex<'a> {
    // 访问redis
    redis_pool: &'a State<Pool>,
    set_options: SetOptions,
}

#[async_trait]
impl<'r> FromRequest<'r> for RedisMutex<'r> {
    type Error = anyhow::Error;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let redis_pool = match req.guard::<&State<Pool>>().await {
            Outcome::Success(redis_pool) => redis_pool,
            _ => return Outcome::Error((Status::BadRequest, anyhow!("redis pool not found"))),
        };

        let m = RedisMutex::new(redis_pool, 30);

        Outcome::Success(m)
    }
}

impl<'a> RedisMutex<'a> {
    pub fn new(redis_pool: &'a State<Pool>, expire: usize) -> Self {
        Self {
            redis_pool,
            set_options: SetOptions::default()
                .conditional_set(ExistenceCheck::NX)
                .with_expiration(SetExpiry::EX(expire)),
        }
    }

    pub async fn lock(&self, key: &str) -> Result<()> {
        if self.redis_pool.get().await?.set_options(key, "lock", self.set_options).await? {
            return Ok(());
        }

        bail!("lock failed");
    }

    pub async fn unlock(&self, key: &str) -> Result<()> {
        self.redis_pool.get().await?.del(key).await?;

        Ok(())
    }
}
