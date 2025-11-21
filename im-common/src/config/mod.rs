use figment::Figment;
use figment::providers::{Env, Format, Toml};
use serde::Deserialize;
use std::sync::LazyLock;

mod db_config;
pub use db_config::DbConfig;
mod redis_config;
pub use redis_config::RedisConfig;

pub static CONFIG: LazyLock<ServerConfig> = LazyLock::new(init);

pub fn init() -> ServerConfig {
    let raw_config = Figment::new()
        .merge(Toml::file(
            Env::var("APP_CONFIG").as_deref().unwrap_or("config.toml"),
        ))
        .merge(Env::prefixed("APP_").global());

    let mut config = match raw_config.extract::<ServerConfig>() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("It looks like your config is invalid. The following error occurred: {e}");
            std::process::exit(1);
        }
    };
    if config.db.url.is_empty() {
        config.db.url = std::env::var("DATABASE_URL").unwrap_or_default();
    }
    if config.db.url.is_empty() {
        eprintln!("DATABASE_URL is not set");
        std::process::exit(1);
    }

    config
}

/// 服务器配置
#[derive(Deserialize, Clone, Debug)]
#[serde(default)]
pub struct ServerConfig {
    /// 监听地址
    pub listen_addr: String,

    /// 数据库配置
    pub db: DbConfig,

    /// Redis 配置
    pub redis: RedisConfig,

    /// JWT 配置
    pub jwt: JwtConfig,

    /// TLS 配置（可选）
    pub tls: Option<TlsConfig>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            listen_addr: "127.0.0.1:8008".into(),
            db: DbConfig::default(),
            redis: RedisConfig::default(),
            jwt: JwtConfig::default(),
            tls: None,
        }
    }
}

/// JWT 配置
#[derive(Deserialize, Clone, Debug)]
pub struct JwtConfig {
    /// JWT 签名密钥
    pub secret: String,

    /// 过期时间（秒）
    pub expiry: i64,
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            secret: "your-secret-key".into(),
            expiry: 3600,
        }
    }
}

/// TLS 配置
#[derive(Deserialize, Clone, Debug)]
pub struct TlsConfig {
    /// 证书文件路径
    pub cert: String,

    /// 私钥文件路径
    pub key: String,
}
