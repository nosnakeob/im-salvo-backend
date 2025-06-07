use crate::domain::user::User;
use crate::{build_salvo, ApiResponse};
use anyhow::Result;
use salvo::test::{ResponseExt, TestClient};
use serde_json::Value;

#[tokio::test]
async fn register() {
    let service = build_salvo().await.unwrap();
    let url = "http://localhost:8000/";

    // 用户名已存在
    let user = User::default();

    let res: ApiResponse<()> = TestClient::post(format!("{}auth/register", url))
        .json(&user)
        .send(&service)
        .await
        .take_json()
        .await
        .unwrap();

    assert!(res.is_error());
}

#[tokio::test]
async fn login() -> Result<()> {
    // 已有对应用户
    let service = build_salvo().await?;
    let url = "http://localhost:8000/";

    let res: ApiResponse<Value> = TestClient::post(format!("{}auth/login", url))
        .json(&User::default())
        .send(&service)
        .await
        .take_json()
        .await?;

    let token = res
        .unwrap()
        .data
        .get("token")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();

    let res: ApiResponse<()> = TestClient::get(format!("{}auth/check", url))
        .add_header("Authorization", format!("Bearer {}", token), true)
        .send(&service)
        .await
        .take_json()
        .await?;

    assert!(res.is_success());

    Ok(())
}
