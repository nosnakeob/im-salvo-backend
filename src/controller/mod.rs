use crate::domain::resp::R;

pub mod auth;
pub mod chat;

rocket_base_path!("/");

#[utoipa::path]
#[get("/")]
pub async fn index() -> R {
    R::ok(None)
}
