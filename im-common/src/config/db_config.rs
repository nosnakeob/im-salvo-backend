use serde::{Deserialize, Serialize};

/// 数据库配置
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(default)]
pub struct DbConfig {
    /// 数据库连接 URL
    #[serde(alias = "database_url")]
    pub url: String,

    /// 连接池大小
    // #[serde]
    pub pool_size: u32,

    /// 最小空闲连接数
    pub min_idle: Option<u32>,

    /// TCP 超时时间（毫秒）
    /// 等待未确认 TCP 数据包的时间，超时后视为连接断开
    pub tcp_timeout: u64,

    /// 连接超时时间（毫秒）
    /// 从连接池获取连接的等待时间
    pub connection_timeout: u64,

    /// 语句超时时间（毫秒）
    /// 等待查询响应的时间，超时后取消查询
    pub statement_timeout: u64,

    /// 辅助线程数
    /// 用于异步操作（如连接创建）的线程数
    pub helper_threads: usize,

    /// 是否强制使用 TLS 加密连接
    pub enforce_tls: bool,
}

impl Default for DbConfig {
    fn default() -> Self {
        Self {
            url: String::new(),
            pool_size: 10,
            min_idle: None,
            tcp_timeout: 10000,
            connection_timeout: 30000,
            statement_timeout: 30000,
            helper_threads: 10,
            enforce_tls: false,
        }
    }
}
