use std::time::Duration;
use salvo::prelude::*;
use web_server::build_salvo;

#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();

    // 构建Salvo服务
    let service = build_salvo().await;

    // 创建监听器并绑定端口
    let acceptor = TcpListener::new("0.0.0.0:8000").bind().await;

    let server = Server::new(acceptor);

    // 监听Ctrl-C信号优雅停机
    let handle = server.handle();
    tokio::spawn(async move {
        if tokio::signal::ctrl_c().await.is_ok() {
            handle.stop_graceful(Duration::from_secs(1));
        }
    });

    // 启动服务器
    server.serve(service).await;
}
