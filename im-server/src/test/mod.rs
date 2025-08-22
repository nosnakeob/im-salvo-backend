use crate::build_salvo;
use crate::models::resp::ApiResponse;
use anyhow::Result;
use im_common::config::CONFIG;
use salvo::test::{ResponseExt, TestClient};

mod auth;
mod chat;

#[tokio::test]
async fn index() -> Result<()> {
    let service = build_salvo().await?;

    let res: ApiResponse<String> = TestClient::get(format!("http://{}", CONFIG.listen_addr))
        .send(&service)
        .await
        .take_json()
        .await?;

    assert_eq!(res.unwrap(), "Hello, world!");

    Ok(())
}
