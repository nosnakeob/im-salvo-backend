use rocket;
use rocket::{Build, Rocket};

pub mod catcher;

pub struct Server(pub Rocket<Build>);

impl Default for Server {
    fn default() -> Self {
        Self { 0: Rocket::build() }
    }
}