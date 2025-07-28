// 用户认证相关的路由处理器

use crate::ApiResponse;
use crate::models::user::User;
use api_response::prelude::*;
use bcrypt::{DEFAULT_COST, hash, verify};
use im_codegen::bail;
use im_common::jwt::{JwtClaims, SECRET_KEY};
use jsonwebtoken::EncodingKey;
use rbatis::RBatis;
use salvo::oapi::extract::JsonBody;
use salvo::prelude::*;
use serde_json::{Value, json};
use time::Duration;

/// 用户注册端点
///
/// # 参数
/// * `json` - 包含用户注册信息的 JSON 数据
/// * `depot` - Salvo 依赖注入容器
///
/// # 返回值
/// 注册成功返回空响应，失败返回错误信息
#[endpoint]
pub async fn register(json: JsonBody<User>, depot: &mut Depot) -> ApiResponse<()> {
    // 从依赖容器中获取数据库连接
    let rb = depot.obtain_mut::<RBatis>().unwrap();

    let mut register_user = json.into_inner();

    // 检查用户名是否已存在
    if User::select_by_name(rb, &register_user.username)
        .await
        .unwrap()
        .is_some()
    {
        bail!("用户名已存在");
    }

    // 对密码进行哈希加密
    register_user.password = hash(register_user.password, DEFAULT_COST).unwrap();

    // 将用户信息插入数据库
    User::insert(rb, &register_user).await.unwrap();

    ().api_response_without_meta()
}

/// 用户登录端点
///
/// # 参数
/// * `json` - 包含用户名和密码的登录信息
/// * `depot` - Salvo 依赖注入容器
///
/// # 返回值
/// 登录成功返回包含 JWT token 的响应，失败返回错误信息
#[endpoint]
pub async fn login(json: JsonBody<User>, depot: &Depot) -> ApiResponse<Value> {
    // 从依赖容器中获取数据库连接
    let rb = depot.obtain::<RBatis>().unwrap();

    let login_user = json.into_inner();

    // 根据用户名查询用户信息
    let user = match User::select_by_name(rb, &login_user.username).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            bail!("用户不存在");
        }
        Err(e) => {
            bail!(e.to_string());
        }
    };

    // 验证密码是否正确
    if !verify(login_user.password, &user.password).unwrap() {
        bail!("密码错误");
    }

    // 创建 JWT 声明，设置 6 小时过期时间
    let claim = JwtClaims::new(&user.username, Duration::hours(6));

    // 生成 JWT token
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claim,
        &EncodingKey::from_secret(SECRET_KEY.as_bytes()),
    )
    .unwrap();

    // TODO: 将 token 存储到 Redis 中用于会话管理
    // let _: () = pool
    //     .get()
    //     .await
    //     .unwrap()
    //     .set_ex(&token, user, 3600)
    //     .await
    //     .unwrap();

    // 返回包含 token 的响应
    json!({ "token": token }).api_response_without_meta()
}

/// 检查用户认证状态
///
/// 此端点需要有效的 JWT token 才能访问
/// 如果 token 有效，返回成功响应；否则返回 401 未授权
///
/// # 返回值
/// 认证成功返回空响应
#[endpoint]
pub async fn check() -> ApiResponse<()> {
    ().api_response_without_meta()
}
