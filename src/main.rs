use anyhow::Result;
use salvo::prelude::*;

#[handler]
async fn hello() -> Result<&'static str> {
    Ok("Hello World")
}

#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::fmt().init();

    // 构建Salvo路由
    // let router = build_salvo();
    let router = Router::new().get(hello);

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
