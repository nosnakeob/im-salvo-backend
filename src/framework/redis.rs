use rocket_db_pools::Database;
use rocket_db_pools::deadpool_redis::Pool;

#[derive(Database)]
#[database("redis")]
pub struct RedisCache(Pool);