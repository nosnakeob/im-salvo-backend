use rocket::http::Status;
use rocket::response::Responder;
use rocket::yansi::Paint;
use serde::{Deserialize, Serialize};
use serde_json::{to_value, Value};

// json响应
#[derive(Debug, Serialize, Deserialize)]
struct JsonR {
    code: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

// 整个响应
#[derive(Responder)]
#[response(content_type = "json")]
pub struct R {
    inner: (Status, Value),
}

impl R {
    pub fn new<S: Into<String>>(code: Status, msg: Option<S>, data: Option<Value>) -> Self {
        let msg = msg.map(Into::into);
        Self { inner: (code, to_value(JsonR { code, msg, data }).unwrap()) }
    }

    pub fn ok(data: Option<Value>) -> Self {
        Self::new::<String>(Status::Ok, None, data)
    }

    pub fn fail<S: Into<String>>(msg: Option<S>) -> Self {
        Self::new(Status::BadRequest, msg, None)
    }
}


