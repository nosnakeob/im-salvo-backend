#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate rocket;

use controller::auth;
use controller::chat;

use crate::controller::index;

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
        .mount("/", routes![index,auth::register,auth::login,auth::check])
        .mount("/chat", routes![chat::connect,chat::kick,chat::status])
}

