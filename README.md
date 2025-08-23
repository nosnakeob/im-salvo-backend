# IM Salvo Backend

基于 Rust 和 Salvo 框架构建的现代化即时通讯后端服务。

## 项目介绍

IM Salvo Backend 是一个完整的即时通讯后端解决方案，基于 Salvo 0.82 框架构建。项目支持用户认证、实时消息传递、文件共享等核心功能，为即时通讯应用提供高性能、可扩展的后端服务。

## 功能特性

- 🚀 **高性能 Web 服务**: 基于 Salvo 0.82 框架，支持异步处理
- 🔐 **用户认证系统**: JWT token 认证 + bcrypt 密码加密
- 📝 **API 文档**: 自动生成 OpenAPI/Swagger 文档
- 💾 **数据持久化**: PostgreSQL + Rbatis 4.6 ORM
- 📦 **缓存支持**: Redis + deadpool-redis 连接池
- 🔌 **实时通信**: Server-Sent Events (SSE) 支持
- 🌐 **跨域支持**: 内置 CORS 中间件
- 🔄 **优雅停机**: 支持 Ctrl+C 信号优雅停止
- 📊 **日志系统**: 基于 tracing 的结构化日志

## 技术栈

| 类别           | 技术           | 版本                  | 说明                 |
| -------------- | -------------- | --------------------- | -------------------- |
| **语言**       | Rust           | stable | 系统编程语言         |
| **Web 框架**   | Salvo          | 0.82                  | 现代化 Rust Web 框架 |
| **数据库**     | PostgreSQL     | 12+                   | 关系型数据库         |
|                | Rbatis         | 4.6                   | Rust ORM 框架        |
| **缓存**       | Redis          | 6+                    | 内存数据库           |
|                | deadpool-redis | 0.22                  | Redis 连接池         |
| **异步运行时** | Tokio          | 1.47                  | 异步运行时           |
| **序列化**     | Serde          | 1.0                   | 序列化/反序列化      |
| **认证**       | jsonwebtoken   | 9.3                   | JWT 处理             |
|                | bcrypt         | 0.17                  | 密码哈希             |
| **日志**       | tracing        | 0.1                   | 结构化日志           |
| **配置**       | figment        | 0.10                  | 配置管理             |

## 项目结构

```
im-salvo-backend/
├── im-codegen/           # 过程宏和代码生成工具
│   ├── src/
│   └── Cargo.toml
├── im-common/            # 共享工具和通用功能
│   ├── src/
│   │   ├── config/       # 配置管理
│   │   ├── utils/        # 工具函数
│   │   ├── jwt.rs        # JWT 认证工具
│   │   └── redis.rs      # Redis 工具
│   └── Cargo.toml
├── im-server/            # 主服务器应用
│   ├── src/
│   │   ├── hoops/        # 中间件和钩子
│   │   ├── models/       # 数据模型
│   │   ├── routers/      # 路由处理器
│   │   ├── test/         # 测试模块
│   │   └── main.rs       # 应用入口点
│   └── Cargo.toml
├── script/               # 数据库脚本
├── Cargo.toml            # 工作空间配置
├── config.toml           # 应用配置
└── docker-compose.yml    # Docker 服务配置
```

## 快速开始

### 环境要求

- **Rust**: stable 工具链 
- **PostgreSQL**: 12+ 版本
- **Redis**: 6+ 版本

### 安装依赖

```bash
# 确保使用 Rust stable 工具链
rustup default stable

# 克隆项目
git clone <repository-url>
cd im-salvo-backend
```

### 配置服务

1. **启动数据库服务** (使用 Docker)

   ```bash
   docker-compose up -d
   ```

2. **配置应用** (编辑 `config.toml`)

   ```toml
   # HTTP 服务器监听地址
   listen_addr = "127.0.0.1:8008"

   [db]
   # PostgreSQL 数据库连接
   url = "postgres://postgres:135246@127.0.0.1:5432/postgres"

   [jwt]
   # JWT 签名密钥
   secret = "yoursecret"
   expiry = 3600

   [redis]
   # Redis 连接
   url = "redis://localhost:6379/"
   ```

### 运行项目

```bash
# 检查代码
cargo check

# 运行服务器
cargo run

# 运行测试
cargo test
```

### 访问服务

- **API 服务**: http://localhost:8008
- **Swagger 文档**: http://localhost:8008/swagger-ui/
- **OpenAPI 规范**: http://localhost:8008/api-doc/openapi.json

## 开发指南

### 工作空间命令

```bash
# 检查整个工作空间
cargo check --workspace

# 构建特定 crate
cargo build -p im-server

# 运行特定 crate 的测试
cargo test -p im-common

# 查看依赖树
cargo tree
```

### API 开发

1. **添加路由**: 在 `im-server/src/routers/` 中添加新的路由模块
2. **定义模型**: 在 `im-server/src/models/` 中定义数据模型
3. **添加中间件**: 在 `im-server/src/hoops/` 中添加中间件
4. **OpenAPI 注解**: 使用 Salvo 的 `#[endpoint]` 和相关注解

## 核心功能模块

### 用户认证

- 用户注册和登录
- JWT token 生成和验证
- 密码加密存储 (bcrypt)

### 实时通信

- Server-Sent Events (SSE) 支持
- 消息推送和订阅
- 连接状态管理

### 数据管理

- PostgreSQL 数据持久化
- Redis 缓存加速
- 连接池管理

## 贡献指南

1. Fork 项目
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'feat: 添加某个功能'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

### 提交规范

使用 [Conventional Commits](https://www.conventionalcommits.org/) 规范：

- `feat`: 新功能
- `fix`: 修复 bug
- `docs`: 文档更新
- `style`: 代码格式调整
- `refactor`: 代码重构
- `test`: 测试相关
- `chore`: 构建配置等

## 许可证

MIT License
