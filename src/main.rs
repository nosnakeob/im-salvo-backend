use rocket;
use web_server::build_rocket;
use anyhow::Result;

// 用main才能优雅停机
#[rocket::main]
async fn main() -> Result<()> {
    build_rocket()
        .ignite().await?
        .launch().await?;

    Ok(())
}


