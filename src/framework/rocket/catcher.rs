use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::Request;

use crate::framework::rocket::resp::{MsgR, R};

#[catch(default)]
pub async fn default_catcher(status: Status, _: &Request<'_>) -> R {
    R::Other(MsgR::new(status, "server error"))
}

#[catch(401)]
pub async fn unauthorized() -> R {
    R::Other(MsgR::new(Status::Unauthorized, "haven't login"))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("init catcher", |rocket| async {
        rocket.register("/", catchers![default_catcher, unauthorized])
    })
}