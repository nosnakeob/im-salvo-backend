use api_response::ApiResponse;
use salvo::prelude::*;

/// jwt校验
#[handler]
pub async fn auth(depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
    match depot.jwt_auth_state() {
        JwtAuthState::Authorized => {}
        _ => {
            res.render(ApiResponse::<(), ()>::from_error_msg(
                StatusCode::UNAUTHORIZED.as_u16(),
                StatusCode::UNAUTHORIZED.canonical_reason().unwrap(),
            ));
            ctrl.skip_rest()
        }
    };
}
