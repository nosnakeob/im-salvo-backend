use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use anyhow::{anyhow, Result};
use deadpool_redis::Pool;
use redis::{AsyncCommands, ExistenceCheck, SetExpiry, SetOptions};
use rocket::{Request, State};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use tokio::task::JoinHandle;

// 异步锁
#[derive(Clone)]
pub struct RedisMutex {
    // 访问redis
    redis_pool: Pool,
    set_options: SetOptions,

    // 排队时间
    wait_time: Duration,

    // 过期时间
    expire: Duration,

    // 续约 key:dog
    watchdogs: Arc<Mutex<HashMap<String, JoinHandle<Result<()>>>>>,
}

unsafe impl Send for RedisMutex {}

unsafe impl Sync for RedisMutex {}


#[async_trait]
impl<'r> FromRequest<'r> for RedisMutex {
    type Error = anyhow::Error;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.guard::<&State<Pool>>().await {
            Outcome::Success(redis_pool) => Outcome::Success(RedisMutex::new((**redis_pool).clone(), None, None)),
            _ => Outcome::Error((Status::BadRequest, anyhow!("redis pool not found"))),
        }
    }
}

impl RedisMutex {
    pub fn new(redis_pool: Pool, expire: Option<u64>, wait_time: Option<u64>) -> Self {
        let expire = expire.unwrap_or(30);

        Self {
            redis_pool,
            set_options: SetOptions::default()
                .conditional_set(ExistenceCheck::NX)
                .with_expiration(SetExpiry::EX(expire as usize)),
            wait_time: Duration::from_secs(wait_time.unwrap_or(3)),
            expire: Duration::from_secs(expire),
            watchdogs: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn try_lock(&self, key: &str) -> Result<bool> {
        // set key value nx ex expire
        Ok(self.redis_pool.get().await?.set_options(key, "lock", self.set_options).await?)
    }

    pub async fn lock(&self, key: &str) -> Result<()> {
        // 排队时间内申请锁
        tokio::time::timeout(self.wait_time, async {
            let mut interval = tokio::time::interval(Duration::from_millis(100));
            loop {
                interval.tick().await;

                // 成功申请锁
                if self.try_lock(key).await? {
                    let expire = self.expire.as_secs();
                    let pool = self.redis_pool.clone();
                    let key_str = key.to_string();

                    self.watchdogs.lock().unwrap().insert(key.to_string(), tokio::spawn(async move {
                        let mut interval = tokio::time::interval(Duration::from_secs(expire / 3));
                        loop {
                            interval.tick().await;
                            // renew
                            let _: () = pool.get().await?.expire(&key_str, expire as i64).await?;
                        }
                    }));

                    return Ok(());
                }
            }
        }).await?
    }

    pub async fn unlock(&mut self, key: &str) -> Result<()> {
        self.redis_pool.get().await?.del(key).await?;

        if let Some(watchdog) = self.watchdogs.lock().unwrap().remove(key) {
            watchdog.abort();
        }

        Ok(())
    }
}
