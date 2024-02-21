#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate rocket;

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
        .register("/", catchers![catcher::default_catcher, catcher::unauthorized])
        .manage(rb)
}

