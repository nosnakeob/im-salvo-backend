#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate rocket;

use std::collections::HashMap;
use std::sync::Mutex;

use rocket::futures::channel::mpsc::UnboundedSender;
use rocket_ws::Message;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use controller::auth;
use controller::chat;
use domain::resp::R;
use framework::{rocket::catcher, swagger::ApiDoc};
use framework::rbatis::init_sql;

mod domain;
mod common;
mod controller;
mod framework;

#[get("/")]
async fn index() -> R {
    R::ok(None)
}


type ClientMap = Mutex<HashMap<u32, UnboundedSender<Message>>>;

#[launch]
async fn rocket() -> _ {
    let rb = init_sql().await.unwrap();

    let clients: ClientMap = Mutex::new(HashMap::new());

    rocket::build()
        .mount("/", routes![index,auth::register,auth::login,auth::check])
        .mount("/chat", routes![chat::connect])
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .register("/", catchers![catcher::default_catcher, catcher::unauthorized])
        .manage(rb)
        .manage(clients)
}

