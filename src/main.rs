mod error;

#[macro_use]
extern crate rocket;

use error::{not_found, unauthorized};


#[get("/")]
async fn index() -> &'static str {
    "web"
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .register("/", catchers![not_found, unauthorized])
}