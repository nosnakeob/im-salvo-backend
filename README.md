# rust rocket web

一个基于 Rust + Rocket 构建的现代化 Web 服务框架。

## 项目介绍

LeRocket 是一个完整的 Web 后端解决方案，采用 Rust 语言开发，基于 Rocket 框架构建。项目集成了常用的功能组件，提供了一套完整的开发框架。

## 功能特性

- 🚀 基于 Rocket 5.0 的高性能 Web 服务
- 🔐 JWT 认证和权限控制
- 📝 OpenAPI/Swagger 自动文档生成
- 💾 PostgreSQL 数据库支持 (Rbatis ORM)
- 📦 Redis 缓存集成
- 🔌 WebSocket 实时通信支持
- 🌐 跨域 (CORS) 支持
- 🔄 优雅停机支持

## 技术栈

| 类别 | 技术 | 说明 |
|------|------|------|
| **Web 框架** | Rocket | Rust Web 框架 |
| | rocket-cors | 跨域支持 |
| | rocket-jwt | JWT 认证 |
| **数据库** | Rbatis | Rust ORM 框架 |
| | PostgreSQL | 主数据库 |
| | Redis | 缓存数据库 |
| | deadpool-redis | Redis 连接池 |
| **API 文档** | utoipa | OpenAPI 文档生成 |
| | utoipa-swagger-ui | Swagger UI 界面 |
| **WebSocket** | rocket_ws | WebSocket 支持 |
| **工具库** | tokio | 异步运行时 |
| | serde | 序列化/反序列化 |
| | anyhow | 错误处理 |
| | tracing | 日志追踪 |

## 项目结构

```
.
├── src/
│   ├── controller/     # 控制器层
│   ├── domain/        # 领域模型
│   ├── framework/     # 框架组件
│   ├── mapper/       # 数据访问层
│   └── main.rs       # 程序入口
├── web-codegen/      # 代码生成工具
├── web-common/       # 公共组件
├── web-proxy/        # 代理服务
└── web-pingora/      # 高性能代理
```

## 快速开始

1. 环境要求
   - Rust 1.75+
   - PostgreSQL 12+
   - Redis 6+

2. 配置数据库
   ```toml
   # Rocket.toml
   [debug.database.postgres]
   url = "postgres://postgres:password@localhost:5432/le_rocket"
   
   [debug.database.redis]
   url = "redis://localhost/"
   ```

3. 运行项目
   ```bash
   cargo run
   ```

4. 访问接口文档
   ```
   http://localhost:8000/swagger-ui/
   ```

## 许可证

MIT License
