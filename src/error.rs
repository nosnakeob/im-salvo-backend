use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;
use rocket::http::Status;
use rocket::Request;

#[catch(default)]
pub async fn default_catcher(status: Status, _: &Request<'_>) -> Value {
    json!({
        "code": status,
        "msg": status.reason()
    })
}

#[catch(401)]
pub async fn not_authorized() -> Value {
    json!({
        "code": Status::Unauthorized,
        "msg": "haven't login"
    })
}
