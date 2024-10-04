#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate web_codegen;

use anyhow::Result;
use rocket::fairing::AdHoc;
use rocket::{Build, Rocket};
use rocket_cors::CorsOptions;
use web_common::core::AppConfig;

mod domain;
mod controller;
mod framework;
mod mapper;

#[cfg(test)]
mod test;


#[auto_mount("src/controller")]
// #[rocket::launch]
pub fn build_rocket() -> Rocket<Build> {
    let cors = CorsOptions::default().to_cors().unwrap();

    rocket::build()
        .attach(web_common::rbatis::stage())
        .attach(framework::swagger::stage())
        .attach(web_common::core::catcher::stage())
        .attach(framework::chat::stage())
        .attach(web_common::redis::stage())
        .attach(AdHoc::config::<AppConfig>())
        .attach(cors)
}

// 用main才能优雅停机
#[rocket::main]
async fn main() -> Result<()> {
    build_rocket()
        .ignite().await?
        .launch().await?;

    Ok(())
}


