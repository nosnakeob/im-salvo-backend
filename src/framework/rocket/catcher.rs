use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::Request;

use crate::domain::resp::R;

#[catch(default)]
pub async fn default_catcher(status: Status, _: &Request<'_>) -> R {
    R::other_err(status, status.reason().unwrap())
}

#[catch(401)]
pub async fn unauthorized() -> R {
    R::other_err(Status::Unauthorized, "haven't login")
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("init catcher", |rocket| async {
        rocket.register("/", catchers![default_catcher, unauthorized])
    })
}