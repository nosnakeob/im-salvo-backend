//! Salvo框架示例代码
//! 展示如何使用Salvo框架实现与当前Rocket项目相似的功能

use salvo::prelude::*;
use salvo::jwt_auth::{ConstDecoder, HeaderFinder, JwtAuth, JwtAuthState, JwtAuthDepotExt};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, EncodingKey, Header};
use time::{Duration, OffsetDateTime};

// JWT认证相关结构和常量
const SECRET_KEY: &str = "your_secret_key"; // 在生产环境中应该使用安全的密钥管理方案

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    username: String,
    exp: i64,
}

// 响应结构
#[derive(Debug, Serialize, Deserialize)]
pub struct Resp {
    pub code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl Resp {
    pub fn new<S: ToString, D: Serialize>(code: u16, msg: Option<S>, data: Option<D>) -> Self {
        Self {
            code,
            msg: msg.map(|s| s.to_string()),
            data: data.map(|d| serde_json::to_value(d).unwrap()),
        }
    }
}

// 用户模型
#[derive(Debug, Serialize, Deserialize, Clone, Extractible)]
#[salvo(extract(default_source(from = "body")))]
pub struct User {
    pub id: Option<u32>,
    pub username: String,
    pub password: String,
}

// 密码处理工具函数
mod password {
    use md5::compute;

    pub fn encode(raw_password: &str) -> String {
        let digest = compute(raw_password);
        format!("{:x}", digest)
    }

    pub fn verify(password: &str, raw_password: &str) -> bool {
        let hashed = encode(raw_password);
        password == hashed
    }
}

// 处理函数
#[handler]
async fn index(res: &mut Response) {
    let resp = Resp::new(200, None::<String>, Some("Hello, world!"));
    res.render(Json(resp));
}

#[handler]
async fn register(req: &mut Request, res: &mut Response) {
    let mut user = req.extract::<User>().await.unwrap();
    
    // 在实际应用中，这里应该检查用户是否已存在
    // 例如：let existing_user = User::select_by_name(&user.username).await?;
    
    // 加密密码
    user.password = password::encode(&user.password);
    
    // 在实际应用中，这里应该将用户保存到数据库
    // 例如：User::insert(&user).await?;
    
    let resp = Resp::new(200, None::<String>, None::<serde_json::Value>);
    res.render(Json(resp));
}

#[handler]
async fn login(req: &mut Request, res: &mut Response) {
    let login_user = req.extract::<User>().await.unwrap();
    
    // 在实际应用中，这里应该从数据库查询用户
    // 例如：let user = User::select_by_name(&login_user.username).await?;
    
    // 模拟用户查询结果
    let user = User {
        id: Some(1),
        username: login_user.username.clone(),
        password: password::encode("123456"), // 假设正确密码是123456
    };
    
    // 验证密码
    if !password::verify(&user.password, &login_user.password) {
        let resp = Resp::new(400, Some("password error"), None::<serde_json::Value>);
        res.render(Json(resp));
        return;
    }
    
    // 创建JWT令牌
    let exp = OffsetDateTime::now_utc() + Duration::days(14);
    let claims = JwtClaims {
        username: user.username,
        exp: exp.unix_timestamp(),
    };
    
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET_KEY.as_bytes()),
    ).unwrap();
    
    // 在实际应用中，这里应该将令牌保存到Redis
    // 例如：redis_pool.get().await?.set_ex(token2key(&token), user, 3600).await?;
    
    let resp = Resp::new(200, None::<String>, Some(serde_json::json!({ "token": token })));
    res.render(Json(resp));
}

#[handler]
async fn check(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    match depot.jwt_auth_state() {
        JwtAuthState::Authorized => {
            let claims = depot.jwt_auth_data::<JwtClaims>().unwrap();
            let resp = Resp::new(200, None::<String>, Some(serde_json::json!({
                "user": {
                    "username": claims.claims.username
                }
            })));
            res.render(Json(resp));
        }
        _ => {
            let resp = Resp::new(401, Some("unauthorized"), None::<serde_json::Value>);
            res.render(Json(resp));
        }
    }
}

// 主函数
#[tokio::main]
async fn main() {
    // 创建JWT认证中间件
    let auth_handler: JwtAuth<JwtClaims, _> = JwtAuth::new(ConstDecoder::from_secret(SECRET_KEY.as_bytes()))
        .finders(vec![Box::new(HeaderFinder::new())])
        .force_passed(false); // 认证失败时不允许请求继续
    
    // 创建路由
    let router = Router::new()
        .get("/", index)
        .push(
            Router::with_path("auth")
                .post("/register", register)
                .post("/login", login)
                .push(
                    Router::with_path("check")
                        .hoop(auth_handler)
                        .get(check)
                )
        );
    
    // 启动服务器
    let acceptor = TcpListener::new("0.0.0.0:8000").bind().await;
    Server::new(acceptor).serve(router).await;
}