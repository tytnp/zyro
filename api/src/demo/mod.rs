mod sys_role_api;


use crate::AppState;
use axum::routing::get;
use axum::Router;

#[allow(dead_code)]
pub fn register_api(app: Router<AppState>) -> Router<AppState> {
    app
        .route("/role/add", get(sys_role_api::add))
        .route("/role/del", get(sys_role_api::del))
        .route("/role/edit", get(sys_role_api::edit))
        .route("/role/list", get(sys_role_api::list))
        .route("/role/one", get(sys_role_api::one))
}