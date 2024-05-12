use std::convert::Infallible;
use std::ops::FromResidual;

use rocket::http::Status;
use rocket::Request;
use rocket::response::Responder;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use serde_json::{to_value, Value};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize)]
pub struct Resp {
    pub code: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl Resp {
    pub fn new<S: ToString, D: Serialize>(code: Status, msg: Option<S>, data: Option<D>) -> Self {
        Self {
            code,
            msg: msg.map(|s| s.to_string()),
            data: data.map(|d| to_value(d).unwrap()),
        }
    }
}


impl<'r> Responder<'r, 'static> for Resp {
    fn respond_to(self, request: &'r Request) -> rocket::response::Result<'static> {
        Json(self).respond_to(request)
    }
}


// 全部可能的响应
#[derive(Debug, Responder, ToSchema)]
#[response(content_type = "json")]
pub enum R {
    #[response(status = 200)]
    Success(Resp),
    // 可预知的错误
    #[response(status = 400)]
    Fail(Resp),
    // 未处理的错误
    #[response(status = 500)]
    Err(Resp),
    // 捕获状态码
    Catch(Resp),
}

impl R {
    pub fn success<T: Serialize>(data: T) -> Self {
        R::Success(Resp::new(Status::Ok, None::<String>, Some(data)))
    }

    pub fn no_val_success() -> Self {
        R::Success(Resp::new(Status::Ok, None::<String>, None::<Value>))
    }

    pub fn fail(msg: &str) -> Self {
        R::Fail(Resp::new(Status::BadRequest, Some(msg), None::<Value>))
    }

    pub fn catch(code: Status, msg: &str) -> Self {
        R::Catch(Resp::new(code, Some(msg), None::<Value>))
    }
}


// accept `?`
impl<E: ToString> FromResidual<Result<Infallible, E>> for R {
    fn from_residual(residual: Result<Infallible, E>) -> Self {
        R::Err(Resp::new(Status::InternalServerError, Some(residual.unwrap_err().to_string()), None::<Value>))
    }
}

#[macro_export]
macro_rules! bail {
    ($e:expr) => {
        return $crate::core::resp::R::fail($e)
    };
}
