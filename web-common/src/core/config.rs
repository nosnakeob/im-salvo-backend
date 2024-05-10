use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RedisConfig {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct PostgresConfig {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct DataBaseConfig {
    pub postgres: PostgresConfig,
    pub redis: RedisConfig,
}
