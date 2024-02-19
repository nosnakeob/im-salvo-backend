use rocket::http::Status;
use rocket::Request;

use crate::domain::req::R;

#[catch(default)]
pub async fn default_catcher(status: Status, _: &Request<'_>) -> R {
    R::new(status, status.reason(), None)
}

#[catch(401)]
pub async fn unauthorized() -> R {
    R::new(Status::Unauthorized, Some("haven't login"), None)
}
