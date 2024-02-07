mod auth;
mod error;

#[macro_use]
extern crate rocket;

use auth::{login, check};
use error::default_catcher;

#[get("/")]
async fn index() -> &'static str {
    "web"
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, login, check])
        .register("/", catchers![default_catcher])
}