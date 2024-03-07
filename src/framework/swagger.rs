use rocket::fairing::AdHoc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::controller::*;
use crate::domain::{resp::R, user::User};

#[derive(OpenApi)]
#[openapi(
paths(
index, pool,
auth::register, auth::login, auth::check,
chat::kick, chat::status
),
components(
schemas(User, R),
),
tags(
(name = "le rocket"),
),
)]
pub struct ApiDoc;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("init doc", |rocket| async {
        rocket.mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
    })
}
