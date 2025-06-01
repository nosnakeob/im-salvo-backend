# Rocket 到 Salvo 框架迁移计划

## 项目概述

当前项目是一个基于Rocket框架的Rust Web应用，使用了以下主要组件：

- Rocket Web框架
- Rbatis ORM
- Redis缓存
- JWT认证
- Swagger文档

## 迁移目标

将项目从Rocket框架迁移到Salvo框架，保持现有功能不变，包括：

- 路由和控制器
- 数据库连接和ORM
- Redis缓存
- JWT认证
- 错误处理
- Swagger文档

## 迁移步骤

### 1. 更新依赖

在`Cargo.toml`文件中，将Rocket相关依赖替换为Salvo依赖：

```toml
# 移除以下依赖
# rocket = { version = "0.5", features = ["json", "uuid", "secrets"] }
# rocket_ws = "0.1"
# rocket_cors = "0.6"
# rocket-jwt = "0.5"

# 添加以下依赖
salvo = { version = "0.79", features = ["jwt-auth", "cors", "websocket", "openapi", "serve-static"] }
salvo-jwt-auth = "0.79"
salvo-core = "0.79"
```

### 2. 修改主程序入口

修改`src/main.rs`文件，使用Salvo的服务器启动方式：

```rust
use salvo::prelude::*;
use web_server::build_salvo;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let router = build_salvo();
    let acceptor = TcpListener::new("0.0.0.0:8000").bind().await;
    Server::new(acceptor).serve(router).await;
    Ok(())
}
```

### 3. 修改web-server库

修改`web-server/src/lib.rs`文件，实现`build_salvo`函数：

```rust
#[macro_use]
extern crate rbatis;

use salvo::prelude::*;

pub mod controller;
pub mod domain;
pub mod framework;
pub mod mapper;

#[cfg(test)]
pub mod test;

pub fn build_salvo() -> Router {
    let router = Router::new()
        .hoop(web_common::rbatis::stage())
        .hoop(framework::swagger::stage())
        .hoop(web_common::core::catcher::stage())
        .hoop(web_common::redis::stage());
        
    // 挂载控制器路由
    controller::mount_routes(router)
}
```

### 4. 修改控制器

修改`web-server/src/controller/mod.rs`文件，实现路由挂载函数：

```rust
use salvo::prelude::*;
use web_common::core::resp::R;

pub mod auth;
pub mod chat;
pub mod captcha;
pub mod demo;

#[handler]
pub async fn index() -> R {
    R::success("Hello, world!")
}

#[handler]
pub async fn pool() -> R {
    R::success(rb.get_pool().unwrap().state().await.as_map().unwrap())
}

pub fn mount_routes(router: Router) -> Router {
    router
        .get("/", index)
        .get("/pool", pool)
        .push(auth::routes())
        .push(chat::routes())
        .push(captcha::routes())
        .push(demo::routes())
}
```

### 5. 修改认证控制器

修改`web-server/src/controller/auth.rs`文件，使用Salvo的处理函数和路由：

```rust
use salvo::prelude::*;
use web_common::jwt::UserClaim;
use serde_json::json;
use deadpool_redis::Pool;
use redis::AsyncCommands;

use web_common::{
    bail,
    core::{
        resp::R,
        constant::cache::token2key,
        utils,
    },
};
use web_common::core::constant::cache::id2key;
use crate::domain::user::User;

#[handler]
pub async fn register(req: &mut Request, res: &mut Response) -> R {
    let register_user = req.extract::<User>().await.unwrap();
    
    let user = User::select_by_name(&register_user.username).await.unwrap();

    if user.is_some() {
        bail!("username exists");
    }

    let mut register_user = register_user;
    register_user.password = utils::password::encode(&register_user.password);

    User::insert(&register_user).await.unwrap();

    R::no_val_success()
}

#[handler]
pub async fn login(req: &mut Request, res: &mut Response, depot: &mut Depot) -> R {
    let login_user = req.extract::<User>().await.unwrap();
    let redis_pool = depot.obtain::<Pool>().unwrap();
    
    let user = User::select_by_name(&login_user.username).await.unwrap();

    let user = match user {
        Some(user) => user,
        None => bail!("user not exists")
    };

    if !utils::password::verify(&user.password, &login_user.password) {
        bail!("password error");
    }

    let user_claim = UserClaim::new();

    let token = UserClaim::sign(user_claim);

    // token -> user 登录鉴权
    redis_pool.get().await.unwrap().set_ex(token2key(&token), user, 3600).await.unwrap();

    R::success(json!({ "token": token }))
}

#[handler]
pub async fn check(req: &mut Request, depot: &mut Depot) -> R {
    let user = depot.obtain::<User>().unwrap();
    R::success(json!({ "user": user }))
}

pub fn routes() -> Router {
    Router::with_path("auth")
        .post("/register", register)
        .post("/login", login)
        .get("/check", check)
}
```

### 6. 修改JWT认证

修改`web-common/src/jwt.rs`文件，使用Salvo的JWT认证：

```rust
use derive_new::new;
use salvo::jwt_auth::{ConstDecoder, HeaderFinder};
use salvo::prelude::*;
use serde::{Deserialize, Serialize};

// 使用环境变量或配置文件获取密钥
const SECRET_KEY: &str = "your_secret_key"; // 在生产环境中应该使用安全的密钥管理方案

#[derive(Debug, Serialize, Deserialize, new)]
pub struct UserClaim {
    exp: i64,
}

impl UserClaim {
    pub fn sign(claim: UserClaim) -> String {
        use jsonwebtoken::{encode, EncodingKey, Header};
        encode(
            &Header::default(),
            &claim,
            &EncodingKey::from_secret(SECRET_KEY.as_bytes()),
        )
        .unwrap()
    }
    
    pub fn create_auth_middleware() -> JwtAuth<UserClaim, ConstDecoder> {
        JwtAuth::new(ConstDecoder::from_secret(SECRET_KEY.as_bytes()))
            .finders(vec![Box::new(HeaderFinder::new())])
    }
}
```

### 7. 修改错误处理

修改`web-common/src/core/catcher.rs`文件，使用Salvo的错误处理：

```rust
use salvo::prelude::*;
use salvo::http::StatusCode;

use super::resp::R;

pub fn stage() -> Handler {
    let handler = |req: &mut Request, res: &mut Response| async move {
        let status = res.status_code().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        let message = status.canonical_reason().unwrap_or("Unknown Error");
        
        res.render(R::catch(status.as_u16(), message));
    };
    
    handler.into_handler()
}
```

### 8. 修改Swagger文档

修改`web-server/src/framework/swagger.rs`文件，使用Salvo的OpenAPI支持：

```rust
use salvo::prelude::*;
use salvo::swagger::{self, SwaggerUI};

use crate::controller::*;
use crate::domain::user::User;
use web_common::core::resp::R;

pub fn stage() -> Handler {
    swagger::swagger_ui()
}
```

### 9. 修改用户模型

修改`web-server/src/domain/user.rs`文件，使用Salvo的请求提取：

```rust
use deadpool_redis::Pool;
use redis::AsyncCommands;
use redis_macros::{FromRedisValue, ToRedisArgs};

use salvo::prelude::*;
use salvo::http::StatusCode;
use serde::{Deserialize, Serialize};

use web_common::core::constant::cache::token2key;

#[derive(Debug, Serialize, Deserialize, Clone, ToRedisArgs, FromRedisValue, Extractible)]
#[salvo(extract(default_source(from = "body")))]
pub struct User {
    pub id: Option<u32>,
    pub username: String,
    pub password: String,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: Some(1),
            username: "snake".to_string(),
            password: "123123".to_string(),
        }
    }
}

// 实现从请求中提取用户的中间件
pub fn auth_middleware() -> impl Handler {
    |req: &mut Request, depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl| async move {
        let redis_pool = depot.obtain::<Pool>().unwrap();
        
        // 从请求头或查询参数中提取token
        let token = req.headers()
            .get("Authorization")
            .and_then(|auth| auth.to_str().ok())
            .and_then(|auth| auth.strip_prefix("Bearer "))
            .map(|token| token.to_string())
            .or_else(|| {
                req.uri().query()
                    .and_then(|query| {
                        query.split('&')
                            .filter_map(|s| s.strip_prefix("Authorization=Bearer%20"))
                            .map(|s| s.to_string())
                            .last()
                    })
            });
        
        if let Some(token) = token {
            let mut conn = redis_pool.get().await.unwrap();
            let user: Option<User> = conn.get(token2key(&token)).await.unwrap();
            
            if let Some(user) = user {
                depot.insert(user);
                return;
            }
        }
        
        res.status_code(StatusCode::UNAUTHORIZED);
        ctrl.skip_rest();
    }
}
```

### 10. 修改响应结构

修改`web-common/src/core/resp.rs`文件，使用Salvo的响应：

```rust
use std::convert::Infallible;
use std::ops::FromResidual;

use salvo::prelude::*;
use salvo::http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::{to_value, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct Resp {
    pub code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl Resp {
    pub fn new<S: ToString, D: Serialize>(code: u16, msg: Option<S>, data: Option<D>) -> Self {
        Self {
            code,
            msg: msg.map(|s| s.to_string()),
            data: data.map(|d| to_value(d).unwrap()),
        }
    }
}

impl Writer for Resp {
    fn write(self, res: &mut Response) {
        res.render(Json(self));
    }
}

// 全部可能的响应
pub enum R {
    Success(Resp),
    // 可预知的错误
    Fail(Resp),
    // 未处理的错误
    Err(Resp),
    // 捕获状态码
    Catch(Resp),
}

impl R {
    pub fn success<D: Serialize>(data: D) -> Self {
        Self::Success(Resp::new(200, None::<String>, Some(data)))
    }

    pub fn no_val_success() -> Self {
        Self::Success(Resp::new(200, None::<String>, None::<Value>))
    }

    pub fn fail<S: ToString>(msg: S) -> Self {
        Self::Fail(Resp::new(400, Some(msg), None::<Value>))
    }

    pub fn err<S: ToString>(msg: S) -> Self {
        Self::Err(Resp::new(500, Some(msg), None::<Value>))
    }

    pub fn catch<S: ToString>(code: u16, msg: S) -> Self {
        Self::Catch(Resp::new(code, Some(msg), None::<Value>))
    }
}

impl Writer for R {
    fn write(self, res: &mut Response) {
        match self {
            R::Success(resp) => {
                res.status_code(StatusCode::OK);
                resp.write(res);
            }
            R::Fail(resp) => {
                res.status_code(StatusCode::BAD_REQUEST);
                resp.write(res);
            }
            R::Err(resp) => {
                res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                resp.write(res);
            }
            R::Catch(resp) => {
                if let Ok(code) = StatusCode::from_u16(resp.code) {
                    res.status_code(code);
                } else {
                    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                }
                resp.write(res);
            }
        }
    }
}
```

## 迁移注意事项

1. Salvo和Rocket的路由系统有所不同，Salvo使用更灵活的树形路由系统
2. Salvo的中间件系统与处理函数使用相同的接口，这与Rocket的fairing系统不同
3. Salvo的请求处理函数接收`Request`、`Response`和`Depot`参数，而不是像Rocket那样使用参数提取
4. Salvo的错误处理方式与Rocket不同，需要适配现有的错误处理逻辑
5. Salvo的JWT认证需要使用`salvo-jwt-auth`库，而不是`rocket-jwt`

## 测试计划

1. 迁移完成后，测试所有API端点
2. 测试JWT认证功能
3. 测试数据库连接和ORM功能
4. 测试Redis缓存功能
5. 测试错误处理功能
6. 测试Swagger文档功能

## 回滚计划

如果迁移过程中遇到无法解决的问题，可以通过以下步骤回滚：

1. 切换回master分支
2. 删除migrate-to-salvo分支
3. 重新创建一个新的迁移分支，采用不同的迁移策略