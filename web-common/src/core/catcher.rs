use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::Request;

use super::resp::R;

#[catch(default)]
async fn default_catcher(status: Status, _: &Request<'_>) -> R {
    R::catch(status, status.reason().unwrap_or_default())
}

#[catch(401)]
async fn unauthorized() -> R {
    R::catch(Status::Unauthorized, "haven't login")
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("init catcher", |rocket| async {
        rocket.register("/", catchers![default_catcher, unauthorized])
    })
}