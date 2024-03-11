use crate::framework::rocket::resp::R;

pub mod auth;
pub mod chat;

rocket_base_path!("/");

#[utoipa::path]
#[get("/")]
pub async fn index() -> R {
    R::Success(None::<u8>.into())
}
