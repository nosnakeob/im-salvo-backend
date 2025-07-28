use serde::{Deserialize, Serialize};

/// Redis configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RedisConfig {
    /// Redis connection URL
    #[serde(default = "default_url")]
    pub url: String,

    /// Connection pool size
    #[serde(default = "default_pool_size")]
    pub pool_size: u32,

    /// Connection timeout in milliseconds
    #[serde(default = "default_connection_timeout")]
    pub connection_timeout: u64,
}

fn default_url() -> String {
    "redis://127.0.0.1:6379".into()
}

fn default_pool_size() -> u32 {
    10
}

fn default_connection_timeout() -> u64 {
    5000
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            url: default_url(),
            pool_size: default_pool_size(),
            connection_timeout: default_connection_timeout(),
        }
    }
}