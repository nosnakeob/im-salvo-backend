use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;
use rocket::http::Status;
use rocket::Request;

#[catch(default)]
pub async fn default_catcher(status: Status, req: &Request<'_>) -> Value {
    json!({
        "code": status.code,
        "msg": status.reason()
    })
}

