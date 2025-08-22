use crate::models::resp::*;
use crate::models::user::User;
use im_common::jwt::JwtClaims;
use rbatis::RBatis;
use salvo::prelude::*;

#[endpoint]
pub async fn status(depot: &mut Depot) -> ApiResponse<User> {
    let rb = depot.obtain::<RBatis>().unwrap();
    let claims = &depot.jwt_auth_data::<JwtClaims>().unwrap().claims;

    User::select_by_id(rb, &claims.id)
        .await
        .unwrap()
        .unwrap()
        .api_response()
}
