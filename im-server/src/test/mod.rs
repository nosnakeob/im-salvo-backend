use crate::{ApiResponse, build_salvo};
use anyhow::Result;
use salvo::test::{ResponseExt, TestClient};

mod auth;
// mod chat;

#[tokio::test]
async fn index() -> Result<()> {
    let service = build_salvo().await?;

    let res: ApiResponse<String> = TestClient::get("http://localhost:8000/")
        .send(&service)
        .await
        .take_json()
        .await?;

    assert_eq!(res.unwrap().data, "Hello, world!");

    Ok(())
}
