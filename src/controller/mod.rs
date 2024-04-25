use crate::framework::rocket::resp::R;

pub mod auth;
pub mod chat;
pub mod captcha;
pub mod demo;

rocket_base_path!("/");

#[utoipa::path]
#[get("/")]
pub async fn index() -> R {
    R::success("Hello, world!")
}


#[rb_conn]
#[utoipa::path]
#[get("/pool")]
pub async fn pool() -> R {
    R::success(rb.get_pool().unwrap().state().await.as_map().unwrap())
}
