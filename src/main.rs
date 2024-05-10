#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate web_codegen;

use rocket::fairing::AdHoc;
use web_common::core::AppConfig;

mod domain;
mod controller;
mod framework;
mod mapper;

#[cfg(test)]
mod test;

#[auto_mount("src/controller")]
#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(web_common::rbatis::stage())
        .attach(framework::swagger::stage())
        .attach(web_common::core::catcher::stage())
        .attach(web_common::websocket::stage())
        .attach(web_common::redis::stage())
        .attach(AdHoc::config::<AppConfig>())
}


