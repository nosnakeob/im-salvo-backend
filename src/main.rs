use salvo::prelude::*;
use web_server::build_salvo;

#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();

    // 构建Salvo路由
    let router = build_salvo().await;

    // 创建监听器并绑定端口
    let acceptor = TcpListener::new("0.0.0.0:8000").bind().await;

    let server = Server::new(acceptor);

    let handle = server.handle();

    // 监听Ctrl-C信号优雅停机
    tokio::spawn(async move {
        if tokio::signal::ctrl_c().await.is_ok() {
            handle.stop_graceful(None);
        }
    });

    // 启动服务器
    server.serve(router).await;
}
