use rocket::http::Status;
use rocket::Request;

use crate::domain::resp::R;
use crate::framework::rocket::Server;

#[catch(default)]
pub async fn default_catcher(status: Status, _: &Request<'_>) -> R {
    R::other_err(status, status.reason().unwrap())
}

#[catch(401)]
pub async fn unauthorized() -> R {
    R::other_err(Status::Unauthorized, "haven't login")
}

impl Server {
    pub fn init_catcher(mut self) -> Self {
        self.0 = self.0.register("/", catchers![default_catcher, unauthorized]);
        self
    }
}