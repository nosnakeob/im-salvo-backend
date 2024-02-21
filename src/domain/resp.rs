use rocket::http::Status;
use rocket::serde::{Deserialize, Serialize};
use serde_json::{to_value, Value};
use utoipa::ToSchema;

// json响应
#[derive(Debug, Serialize, Deserialize)]
struct JsonR {
    code: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrR {
    code: Status,
    msg: String,
}

// 整个响应
#[derive(Debug, Responder, ToSchema)]
#[response(content_type = "json")]
pub enum R {
    // inner: (Status, Value),
    #[response(status = 200, content_type = "json")]
    Ok(Value),
    #[response(status = 400, content_type = "json")]
    Fail(Value),
    OtherErr(Value),
}

impl R {
    pub fn ok(data: Option<Value>) -> Self {
        R::Ok(to_value(JsonR {
            code: Status::Ok,
            data,
        }).unwrap())
    }

    pub fn fail<S: Into<String>>(msg: S) -> Self {
        R::Fail(to_value(ErrR {
            code: Status::BadRequest,
            msg: msg.into(),
        }).unwrap())
    }

    pub fn other_err<S: Into<String>>(code: Status, msg: S) -> Self {
        R::OtherErr(to_value(ErrR {
            code,
            msg: msg.into(),
        }).unwrap())
    }
}

