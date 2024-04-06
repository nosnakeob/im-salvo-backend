#![feature(try_trait_v2)]
#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate web_codegen;

use rocket_db_pools::Database;

mod domain;
mod common;
mod controller;
mod framework;
mod mapper;

#[cfg(test)]
mod test;

#[auto_mount("src/controller")]
#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(framework::rbatis::stage())
        .attach(framework::swagger::stage())
        .attach(framework::rocket::catcher::stage())
        .attach(framework::websocket::stage())
        .attach(framework::redis::RedisCache::init())
}


