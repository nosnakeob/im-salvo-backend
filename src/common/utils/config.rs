use rocket::Config;
use rocket::figment::value::Value;

pub fn get_config(key: &str) -> rocket::figment::error::Result<Value> {
    Config::figment().find_value(key)
}