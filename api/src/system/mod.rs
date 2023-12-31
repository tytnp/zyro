mod sys_role_api;
mod sys_user_api;

use crate::AppState;
use axum::routing::get;
use axum::Router;

pub fn register_api(app: Router<AppState>) -> Router<AppState> {
    app.route("/role/add", get(sys_role_api::add))
        .route("/role/del", get(sys_role_api::del))
        .route("/role/edit", get(sys_role_api::edit))
        .route("/role/list", get(sys_role_api::list))
        .route("/role/one", get(sys_role_api::one))
        .route("/user/add", get(sys_user_api::add))
        .route("/user/del", get(sys_user_api::del))
        .route("/user/edit", get(sys_user_api::edit))
        .route("/user/list", get(sys_user_api::list))
        .route("/user/one", get(sys_user_api::one))
}
