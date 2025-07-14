use crate::domain::db::User;
use crate::{ApiResponse, build_salvo};
use anyhow::Result;
use rand::Rng;
use rand::distr::Alphanumeric;
use rbatis::RBatis;
use rbdc_pg::PgDriver;
use salvo::test::{ResponseExt, TestClient};
use serde_json::Value;

#[tokio::test]
async fn test_register_existing_user() {
    let service = build_salvo().await.unwrap();
    let url = "http://localhost:8000/";

    // 用户名已存在
    let user = User::default();

    let res: ApiResponse<()> = TestClient::post(format!("{}/auth/register", url))
        .json(&user)
        .send(&service)
        .await
        .take_json()
        .await
        .unwrap();

    assert!(res.unwrap_err().message().contains("username exists"));
}

#[tokio::test]
async fn test_register_new_user() -> Result<()> {
    let service = build_salvo().await?;
    let url = "http://localhost:8000/";

    // 创建一个新用户，使用随机用户名避免冲突
    let user_name = format!(
        "testuser_{}",
        rand::rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect::<String>()
    );

    let new_user = User {
        id: None,
        username: user_name.clone(),
        password: "password123".to_string(),
        status: None,
        created_at: None,
        updated_at: None,
    };

    let res: ApiResponse<()> = TestClient::post(format!("{}/auth/register", url))
        .json(&new_user)
        .send(&service)
        .await
        .take_json()
        .await?;

    assert!(res.is_success());

    // 验证新注册的用户可以登录
    let login_res: ApiResponse<Value> = TestClient::post(format!("{}/auth/login", url))
        .json(&new_user)
        .send(&service)
        .await
        .take_json()
        .await?;

    assert!(login_res.is_success());

    // 删除测试用户
    let rb = RBatis::new();
    rb.link(PgDriver {}, "postgres://postgres:135246@localhost/postgres")
        .await?;
    User::delete_by_name(&rb, &user_name).await?;

    Ok(())
}

#[tokio::test]
async fn test_login_success() -> Result<()> {
    // 已有对应用户
    let service = build_salvo().await?;
    let url = "http://localhost:8000/";

    let res: ApiResponse<Value> = TestClient::post(format!("{}/auth/login", url))
        .json(&User::default())
        .send(&service)
        .await
        .take_json()
        .await?;

    assert!(res.is_success());

    let token = res
        .unwrap()
        .data
        .get("token")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();

    let res: ApiResponse<()> = TestClient::get(format!("{}/auth/check", url))
        .add_header("Authorization", format!("Bearer {}", token), true)
        .send(&service)
        .await
        .take_json()
        .await?;

    assert!(res.is_success());

    Ok(())
}

#[tokio::test]
async fn test_login_wrong_password() -> Result<()> {
    let service = build_salvo().await?;
    let url = "http://localhost:8000/";

    // 使用正确的用户名但错误的密码
    let user = User {
        id: None,
        username: "alice".to_string(),
        password: "wrong_password".to_string(),
        status: None,
        created_at: None,
        updated_at: None,
    };

    let res: ApiResponse<Value> = TestClient::post(format!("{}/auth/login", url))
        .json(&user)
        .send(&service)
        .await
        .take_json()
        .await?;

    assert!(res.unwrap_err().message().contains("password error"));

    Ok(())
}

#[tokio::test]
async fn test_login_user_not_exists() -> Result<()> {
    let service = build_salvo().await?;
    let url = "http://localhost:8000/";

    // 使用不存在的用户名
    let user = User {
        id: None,
        username: "nonexistent_user".to_string(),
        password: "any_password".to_string(),
        status: None,
        created_at: None,
        updated_at: None,
    };

    let res: ApiResponse<Value> = TestClient::post(format!("{}/auth/login", url))
        .json(&user)
        .send(&service)
        .await
        .take_json()
        .await?;

    assert!(res.unwrap_err().message().contains("user not exists"));

    Ok(())
}

#[tokio::test]
async fn test_unauthorized_access() -> Result<()> {
    let service = build_salvo().await?;
    let url = "http://localhost:8000/";

    // 尝试未授权访问需要认证的端点
    let res: ApiResponse<()> = TestClient::get(format!("{}/auth/check", url))
        .send(&service)
        .await
        .take_json()
        .await?;

    assert!(res.is_error());

    Ok(())
}
