use anyhow::Result;
use deadpool_redis::Config;
use redis::Client;
use salvo::prelude::*;

#[handler]
pub async fn set_redis(depot: &mut Depot) -> Result<()> {
    depot
        .inject(Config::from_url("redis://localhost/").create_pool(None)?)
        // 用于发布订阅
        .inject(Client::open("redis://localhost/")?);
    Ok(())
}
