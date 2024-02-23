#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate rocket;

use controller::auth;
use controller::chat;
use domain::resp::R;

use crate::framework::rocket::Server;

mod domain;
mod common;
mod controller;
mod framework;

#[get("/")]
async fn index() -> R {
    R::ok(None)
}


#[launch]
async fn rocket() -> _ {
    Server::default()
        .init_sql().await
        .init_doc()
        .init_chat()
        .init_catcher()
        .inner
        .mount("/", routes![index,auth::register,auth::login,auth::check])
        .mount("/chat", routes![chat::connect])
}

