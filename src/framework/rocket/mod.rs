use serde::Deserialize;
use self::config::DataBaseConfig;

pub mod catcher;
pub mod resp;
mod config;


#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub database: DataBaseConfig,
}