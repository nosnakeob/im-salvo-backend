#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate web_codegen;

use controller::{auth, chat};

mod domain;
mod common;
mod controller;
mod framework;
mod mapper;

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(framework::rbatis::stage())
        .attach(framework::swagger::stage())
        .attach(framework::rocket::catcher::stage())
        .attach(framework::websocket::stage())
        .attach(controller::routes())
        .attach(auth::routes())
        .attach(chat::routes())
}


