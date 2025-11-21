use serde::{Deserialize, Serialize};

/// Redis 配置
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct RedisConfig {
    /// Redis 连接 URL
    pub url: String,

    /// 连接池大小
    pub pool_size: u32,

    /// 连接超时时间（毫秒）
    pub connection_timeout: u64,
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            url: "redis://127.0.0.1:6379".into(),
            pool_size: 10,
            connection_timeout: 5000,
        }
    }
}