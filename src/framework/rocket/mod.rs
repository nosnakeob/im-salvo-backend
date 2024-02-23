use rocket;
use rocket::{Build, Rocket};

pub mod catcher;

pub struct Server {
    pub inner: Rocket<Build>,
}

impl Default for Server {
    fn default() -> Self {
        Self { inner: Rocket::build() }
    }
}