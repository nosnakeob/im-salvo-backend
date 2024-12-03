#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate web_codegen;

use rocket::fairing::AdHoc;
use rocket::{Build, Rocket};
use rocket_cors::CorsOptions;
use web_common::core::AppConfig;

pub mod controller;
pub mod domain;
pub mod framework;
pub mod mapper;

#[cfg(test)]
pub mod test;

#[auto_mount("web-server/src/controller")]
pub fn build_rocket() -> Rocket<Build> {
    rocket::build()
        .attach(web_common::rbatis::stage())
        .attach(framework::swagger::stage())
        .attach(web_common::core::catcher::stage())
        .attach(web_common::redis::stage())
        .attach(AdHoc::config::<AppConfig>())
        .attach(CorsOptions::default().to_cors().unwrap())
}
