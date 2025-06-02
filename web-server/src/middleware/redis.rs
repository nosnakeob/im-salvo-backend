use anyhow::Result;
use deadpool_redis::Config;
use salvo::prelude::*;

#[handler]
pub async fn set_redis_pool(depot: &mut Depot) -> Result<()> {
    depot.inject(Config::from_url("redis://localhost/").create_pool(None)?);
    Ok(())
}
