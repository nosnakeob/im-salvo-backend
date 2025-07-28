// 用户认证相关的测试模块

use crate::models::user::User;
use crate::{ApiResponse, build_salvo};
use anyhow::Result;
use im_common::config::CONFIG;
use rand::Rng;
use rand::distr::Alphanumeric;
use rbatis::RBatis;
use rbdc_pg::PgDriver;
use salvo::http::StatusCode;
use salvo::test::{ResponseExt, TestClient};
use serde_json::Value;

/// 测试注册已存在的用户名
/// 应该返回用户名已存在的错误
#[tokio::test]
async fn test_register_existing_user() {
    // 构建测试服务
    let service = build_salvo().await.unwrap();

    // 使用默认用户（用户名已存在）
    let user = User::default();

    // 发送注册请求
    let res: ApiResponse<()> =
        TestClient::post(format!("http://{}/auth/register", CONFIG.listen_addr))
            .json(&user)
            .send(&service)
            .await
            .take_json()
            .await
            .unwrap();

    assert!(res.is_error());
}

/// 测试注册新用户
/// 应该成功注册并能够登录，测试完成后清理数据
#[tokio::test]
async fn test_register_new_user() -> Result<()> {
    // 构建测试服务
    let service = build_salvo().await?;

    // 生成随机用户名避免冲突
    let user_name = format!(
        "testuser_{}",
        rand::rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect::<String>()
    );

    // 创建新用户数据
    let new_user = User {
        id: None,
        username: user_name.clone(),
        password: "password123".to_string(),
        status: None,
        created_at: None,
        updated_at: None,
    };

    // 发送注册请求
    let res: ApiResponse<()> =
        TestClient::post(format!("http://{}/auth/register", CONFIG.listen_addr))
            .json(&new_user)
            .send(&service)
            .await
            .take_json()
            .await?;

    // 验证注册成功
    assert!(res.is_success());

    // 验证新注册的用户可以登录
    let login_res: ApiResponse<Value> =
        TestClient::post(format!("http://{}/auth/login", CONFIG.listen_addr))
            .json(&new_user)
            .send(&service)
            .await
            .take_json()
            .await?;

    // 验证登录成功
    assert!(login_res.is_success());

    // 清理测试数据：删除测试用户
    let rb = RBatis::new();
    rb.link(PgDriver {}, &CONFIG.db.url).await?;
    User::delete_by_name(&rb, &user_name).await?;

    Ok(())
}

/// 测试用户登录成功
/// 使用正确的用户名和密码登录，获取 token 并验证认证
#[tokio::test]
async fn test_login_success() -> Result<()> {
    // 构建测试服务
    let service = build_salvo().await?;

    // 使用默认用户登录（数据库中已存在的用户）
    let res: ApiResponse<Value> =
        TestClient::post(format!("http://{}/auth/login", CONFIG.listen_addr))
            .json(&User::default())
            .send(&service)
            .await
            .take_json()
            .await?;

    // 验证登录成功
    assert!(res.is_success());

    // 从响应中提取 JWT token
    let token = res
        .unwrap()
        .data
        .get("token")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();

    // 使用 token 访问需要认证的端点
    let res: ApiResponse<()> = TestClient::get(format!("http://{}/auth/check", CONFIG.listen_addr))
        .add_header("Authorization", format!("Bearer {}", token), true)
        .send(&service)
        .await
        .take_json()
        .await?;

    // 验证认证成功
    assert!(res.is_success());

    Ok(())
}

/// 测试使用错误密码登录
/// 应该返回密码错误的提示
#[tokio::test]
async fn test_login_wrong_password() -> Result<()> {
    // 构建测试服务
    let service = build_salvo().await?;

    // 使用正确的用户名但错误的密码
    let user = User {
        id: None,
        username: "alice".to_string(),
        password: "wrong_password".to_string(),
        status: None,
        created_at: None,
        updated_at: None,
    };

    // 发送登录请求
    let res: ApiResponse<Value> =
        TestClient::post(format!("http://{}/auth/login", CONFIG.listen_addr))
            .json(&user)
            .send(&service)
            .await
            .take_json()
            .await?;

    assert!(res.is_error());

    Ok(())
}

/// 测试使用不存在的用户名登录
/// 应该返回用户不存在的错误
#[tokio::test]
async fn test_login_user_not_exists() -> Result<()> {
    // 构建测试服务
    let service = build_salvo().await?;

    // 使用不存在的用户名
    let user = User {
        id: None,
        username: "nonexistent_user".to_string(),
        password: "any_password".to_string(),
        status: None,
        created_at: None,
        updated_at: None,
    };

    // 发送登录请求
    let res: ApiResponse<Value> =
        TestClient::post(format!("http://{}/auth/login", CONFIG.listen_addr))
            .json(&user)
            .send(&service)
            .await
            .take_json()
            .await?;

    assert!(res.is_error());

    Ok(())
}

/// 测试未授权访问
/// 不提供 JWT token 访问需要认证的端点，应该返回 401 状态码
#[tokio::test]
async fn test_unauthorized_access() -> Result<()> {
    // 构建测试服务
    let service = build_salvo().await?;

    // 不提供认证信息，直接访问需要认证的端点
    let res = TestClient::get(format!("http://{}/auth/check", CONFIG.listen_addr))
        .send(&service)
        .await;

    // 验证返回 401 未授权状态码
    assert_eq!(res.status_code, Some(StatusCode::UNAUTHORIZED));

    Ok(())
}
