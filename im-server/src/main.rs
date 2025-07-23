#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate tracing;

use crate::hoops::cors_hoop;
use anyhow::Result;
use deadpool_redis::Config;
use im_common::config::CONFIG;
use rbatis::RBatis;
use rbdc_pg::PgDriver;
use redis::Client;
use salvo::prelude::*;
use salvo::server::ServerHandle;
use std::time::Duration;

pub mod hoops;
pub mod models;
pub mod routers;

#[cfg(test)]
pub mod test;

type ApiResponse<T> = api_response::ApiResponse<T, ()>;

/// 构建Salvo应用程序
pub async fn build_salvo() -> Result<Service> {
    let redis_pool = Config::from_url("redis://localhost/").create_pool(None)?;
    // 用于发布订阅
    let redis_client = Client::open("redis://localhost/")?;

    let rb = RBatis::new();
    rb.link(PgDriver {}, &CONFIG.db.url).await?;

    Ok(Service::new(routers::root())
        .hoop(
            affix_state::inject(rb)
                .inject(redis_pool)
                .inject(redis_client),
        )
        .hoop(cors_hoop())
        .hoop(Compression::default()))
}

async fn shutdown_signal(handle: ServerHandle) {
    if tokio::signal::ctrl_c().await.is_ok() {
        handle.stop_graceful(Duration::from_secs(1));
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // 构建Salvo服务
    let service = build_salvo().await?;

    // 创建监听器并绑定端口
    let acceptor = TcpListener::new(&CONFIG.listen_addr).bind().await;

    let server = Server::new(acceptor);

    // 监听Ctrl-C信号优雅停机
    tokio::spawn(shutdown_signal(server.handle()));

    // 启动服务器
    server.serve(service).await;

    Ok(())
}
