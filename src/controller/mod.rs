use crate::domain::resp::R;

pub mod auth;
pub mod chat;

#[get("/")]
pub async fn index() -> R {
    R::ok(None)
}
