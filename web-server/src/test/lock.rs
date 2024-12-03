use anyhow::Result;
use deadpool_redis::Config;
use redis::Script;

use web_common::core::utils::config::get_config;

#[tokio::test]
async fn script() -> Result<()> {
    let url = get_config("database.redis.url")?.into_string().unwrap();
    let pool = Config::from_url(url).create_pool(None)?;

    let script = Script::new(r"return redis.call('set', KEYS[1]..'aa', ARGV[1])");

    script.key("key").arg(10).invoke_async(&mut pool.get().await?).await?;

    Ok(())
}