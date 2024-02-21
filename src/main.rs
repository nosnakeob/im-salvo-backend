#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate rocket;

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


#[launch]
async fn rocket() -> _ {
    let rb = init_sql().await.unwrap();

    rocket::build()
        .mount("/", routes![index,auth::register,auth::login,auth::check])
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .register("/", catchers![catcher::default_catcher, catcher::unauthorized])
        .manage(rb)
}

