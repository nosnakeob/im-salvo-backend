use serde::Deserialize;
use self::config::DataBaseConfig;

mod config;
pub mod catcher;
pub mod resp;
pub mod utils;
pub mod constant;


#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub database: DataBaseConfig,
}
