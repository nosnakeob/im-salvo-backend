use rocket::Config;
use rocket::figment::value::Value;
use anyhow::Result;

pub fn get_config(key: &str) -> Result<Value> {
    Ok(Config::figment().find_value(key)?)
}