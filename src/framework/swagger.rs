use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::controller::auth;
use crate::domain::{resp::R, user::User};
use crate::framework::rocket::Server;

#[derive(OpenApi)]
#[openapi(
paths(
auth::register, auth::login, auth::check
),
components(
schemas(User, R),
),
tags(
(name = "le rocket"),
),
)]
pub struct ApiDoc;

impl Server {
    pub fn init_doc(mut self) -> Self {
        self.inner = self.inner.mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>").url("/api-docs/openapi.json", ApiDoc::openapi()),
        );

        self
    }
}
