// IM Salvo 后端服务主程序

#[macro_use]
extern crate rbatis;
#[macro_use]
extern crate tracing;

use crate::hoops::cors_hoop;
use anyhow::Result;
use im_common::config;
use rbatis::RBatis;
use rbdc_pg::PgDriver;
use redis::Client;
use salvo::prelude::*;
use salvo::server::ServerHandle;
use std::time::Duration;

// 模块声明
pub mod hoops; // 中间件和钩子
pub mod models; // 数据模型
pub mod routers; // 路由处理器

#[cfg(test)]
pub mod test; // 测试模块

/// 构建 Salvo 应用程序
///
/// 初始化数据库连接、Redis 连接池和客户端，配置中间件
///
/// # 返回值
/// 返回配置好的 Salvo 服务实例
pub async fn build_salvo() -> Result<Service> {
    // 创建 Redis 连接池用于缓存
    let redis_pool = deadpool_redis::Config::from_url("redis://localhost/").create_pool(None)?;

    // 创建 Redis 客户端用于发布订阅
    let redis_client = Client::open("redis://localhost/")?;

    // 初始化数据库连接
    let rb = RBatis::new();
    let config = config::init();
    rb.link(PgDriver {}, &config.db.url).await?;

    // 构建服务，注入依赖并配置中间件
    Ok(Service::new(routers::root())
        .hoop(
            affix_state::inject(rb)
                .inject(redis_pool)
                .inject(redis_client),
        )
        .hoop(cors_hoop()) // 跨域支持
        .hoop(Compression::default())) // 响应压缩
}

/// 优雅停机信号处理
///
/// 监听 Ctrl+C 信号，收到信号后优雅地停止服务器
///
/// # 参数
/// * `handle` - 服务器句柄，用于控制服务器停止
async fn shutdown_signal(handle: ServerHandle) {
    if tokio::signal::ctrl_c().await.is_ok() {
        info!("收到停机信号，正在优雅停止服务器...");
        handle.stop_graceful(Duration::from_secs(1));
    }
}

/// 程序主入口点
///
/// 初始化日志系统，构建服务，启动 HTTP 服务器
#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志系统，设置调试级别
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    info!("正在启动 IM Salvo 后端服务...");

    // 加载配置
    let config = config::init();

    // 构建 Salvo 服务
    let service = build_salvo().await?;

    // 创建 TCP 监听器并绑定到配置的地址
    let acceptor = TcpListener::new(config.listen_addr.clone()).bind().await;
    info!("服务器监听地址: {}", config.listen_addr);

    let server = Server::new(acceptor);

    // 在后台任务中监听 Ctrl+C 信号以实现优雅停机
    tokio::spawn(shutdown_signal(server.handle()));

    info!("服务器启动成功！");
    info!("API 文档地址: http://{}/swagger-ui/", config.listen_addr);

    // 启动服务器并开始处理请求
    server.serve(service).await;

    info!("服务器已停止");

    Ok(())
}
