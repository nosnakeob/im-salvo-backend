// API 响应模型

use anyhow::{Result, bail};
use salvo::{
    oapi::{self, Components, Content, RefOr},
    prelude::*,
};
use serde::{Deserialize, Serialize};

/// API 响应枚举
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(untagged)]
pub enum ApiResponse<Data> {
    /// 成功响应
    Success {
        /// 状态码
        code: u16,
        /// 响应数据
        data: Data,
    },
    /// 错误响应
    Error {
        /// 错误代码
        code: u16,
        /// 错误消息
        message: String,
    },
}

impl<Data> ApiResponse<Data> {
    /// 创建成功响应
    pub fn success(data: Data) -> Self {
        Self::Success {
            code: StatusCode::OK.as_u16(),
            data,
        }
    }

    /// 创建错误响应
    pub fn error(code: u16, message: impl Into<String>) -> Self {
        Self::Error {
            code,
            message: message.into(),
        }
    }

    /// 检查是否为成功响应
    pub const fn is_success(&self) -> bool {
        matches!(self, Self::Success { .. })
    }

    /// 检查是否为错误响应
    pub const fn is_error(&self) -> bool {
        matches!(self, Self::Error { .. })
    }

    /// 提取成功响应的数据（可能 panic）
    ///
    /// # Panics
    /// 如果响应是错误类型，将会 panic
    pub fn unwrap(self) -> Data {
        match self {
            Self::Success { code: _, data } => data,
            Self::Error { code, message } => {
                panic!(
                    "调用 ApiResponse::unwrap() 时遇到错误响应: code={}, message={}",
                    code, message
                )
            }
        }
    }

    /// 转换为 Result 类型
    pub fn into_result(self) -> Result<Data> {
        match self {
            Self::Success { code: _, data } => Ok(data),
            Self::Error { code: _, message } => bail!(message),
        }
    }
}

// 便捷的类型转换实现（避免冲突的特定实现）

// 为 anyhow::Error 提供转换
impl<Data> From<anyhow::Error> for ApiResponse<Data> {
    fn from(err: anyhow::Error) -> Self {
        Self::error(500, err.to_string())
    }
}

// 为 Result 类型提供转换
impl<Data, E> From<Result<Data, E>> for ApiResponse<Data>
where
    E: std::fmt::Display,
{
    fn from(result: Result<Data, E>) -> Self {
        match result {
            Ok(data) => Self::success(data),
            Err(err) => Self::error(500, err.to_string()),
        }
    }
}

pub trait ApiSuccessResponse: Sized {
    fn api_response(self) -> ApiResponse<Self> {
        ApiResponse::Success {
            code: 200,
            data: self,
        }
    }
}

impl<Data> ApiSuccessResponse for Data {}

#[async_trait]
impl<Data: Serialize + Send> Scribe for ApiResponse<Data> {
    fn render(self, res: &mut Response) {
        res.render(Json(self));
    }
}

impl<Data> ToResponse for ApiResponse<Data>
where
    Data: ToSchema + 'static,
{
    fn to_response(components: &mut Components) -> RefOr<oapi::Response> {
        oapi::Response::new("Response with json format data")
            .add_content(
                "application/json",
                Content::new(Self::to_schema(components)),
            )
            .into()
    }
}

impl<Data> EndpointOutRegister for ApiResponse<Data>
where
    Data: ToSchema + 'static,
{
    #[inline]
    fn register(components: &mut Components, operation: &mut oapi::Operation) {
        operation
            .responses
            .insert("200", Self::to_response(components));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn t_json() {
        let resp = "hahahi".api_response();

        assert_eq!(
            json!(resp),
            json!({
                "code": 200,
                "data": "hahahi",
            })
        );

        let resp = json!({ "token":"11451" }).api_response();

        assert_eq!(
            json!(resp),
            json!({
                "code": 200,
                "data": {
                    "token": "11451",
                },
            })
        );

        let resp = ApiResponse::<()>::error(StatusCode::UNAUTHORIZED.as_u16(), "fail");

        assert_eq!(
            json!(resp),
            json!({
                "code": StatusCode::UNAUTHORIZED.as_u16(),
                "message": "fail",
            })
        );
    }

    #[test]
    fn t_convert() -> Result<()> {
        let resp = "hahahi".api_response();
        assert_eq!(resp.clone().unwrap(), "hahahi");
        assert_eq!(resp.into_result()?, "hahahi");

        let resp = ApiResponse::<()>::error(StatusCode::UNAUTHORIZED.as_u16(), "fail");
        assert!(resp.into_result().is_err());

        Ok(())
    }
}
