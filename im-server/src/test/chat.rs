use crate::{
    ApiResponse, build_salvo,
    models::{msg::Message, user::User},
};
use anyhow::Result;
use im_common::config::CONFIG;
use salvo::test::{ResponseExt, TestClient};
use serde_json::Value;

#[tokio::test]
async fn test_send_chat_message() -> Result<()> {
    let service = build_salvo().await?;

    let res: ApiResponse<Value> =
        TestClient::post(format!("http://{}/auth/login", CONFIG.listen_addr))
            .json(&User::default())
            .send(&service)
            .await
            .take_json()
            .await?;

    let response_data = res.into_result_data()?;
    let token = response_data.get("token").unwrap().as_str().unwrap();

    let msg = Message::default();

    let res: ApiResponse<()> = TestClient::post(format!("http://{}/chat", CONFIG.listen_addr))
        .add_header("Authorization", format!("Bearer {}", token), true)
        .json(&msg)
        .send(&service)
        .await
        .take_json()
        .await?;

    assert!(res.is_success());

    Ok(())
}
