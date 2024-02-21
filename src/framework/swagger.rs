use utoipa::OpenApi;

use crate::controller::auth;
use crate::domain::{resp::R, user::User};

#[derive(OpenApi)]
#[openapi(
paths(
auth::register, auth::login, auth::check
),
components(
schemas( User, R),
),
tags(
(name = "le rocket"),
),
)]
pub struct ApiDoc;
