use std::convert::Infallible;
use std::error::Error;
use std::ops::FromResidual;

use rocket::http::Status;
use rocket::Request;
use rocket::response::Responder;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use serde_json::{to_value, Value};
use utoipa::ToSchema;

#[derive(Debug, Serialize)]
pub struct DataR {
    code: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<Value>,
}

impl<T: Serialize> From<Option<T>> for DataR {
    fn from(data: Option<T>) -> Self {
        DataR {
            code: Status::Ok,
            data: data.map(|d| to_value(d).unwrap()),
        }
    }
}

impl<'r> Responder<'r, 'static> for DataR {
    fn respond_to(self, request: &'r Request) -> rocket::response::Result<'static> {
        Json(self).respond_to(request)
    }
}

#[derive(Debug, Serialize)]
pub struct MsgR {
    code: Status,
    msg: String,
}

impl MsgR {
    pub fn new<T: ToString>(code: Status, msg: T) -> Self {
        Self { code, msg: msg.to_string() }
    }
}


impl<T: ToString> From<T> for MsgR {
    fn from(msg: T) -> Self {
        MsgR {
            code: Status::BadRequest,
            msg: msg.to_string(),
        }
    }
}

impl<'r> Responder<'r, 'static> for MsgR {
    fn respond_to(self, request: &Request<'_>) -> rocket::response::Result<'static> {
        Json(self).respond_to(request)
    }
}


// 整个响应
#[derive(Debug, Responder, ToSchema)]
#[response(content_type = "json")]
pub enum R {
    #[response(status = 200, content_type = "json")]
    Success(DataR),
    #[response(status = 400, content_type = "json")]
    Fail(MsgR),
    #[response(status = 500, content_type = "json")]
    Err(MsgR),
    Other(MsgR),
}

// accept `?`
impl<E: Error> FromResidual<Result<Infallible, E>> for R {
    fn from_residual(residual: Result<Infallible, E>) -> Self {
        R::Err(MsgR::new(Status::InternalServerError, residual.unwrap_err().to_string()))
    }
}