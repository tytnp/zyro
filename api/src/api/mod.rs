use crate::AppState;
use axum::routing::*;
use axum::Router;
mod sys_api_api;
mod sys_dictionary_api;
mod sys_dictionary_detail_api;
mod sys_menu_api;
mod sys_role_api;

mod sys_user_api;
mod sys_user_role_api;
pub fn register_api(app: Router<AppState>) -> Router<AppState> {
    app.route("/api/add", post(sys_api_api::add))
        .route("/api/del", post(sys_api_api::del))
        .route("/api/edit", post(sys_api_api::edit))
        .route("/api/list", post(sys_api_api::list))
        .route("/api/one", post(sys_api_api::one))
        .route("/dictionary/add", post(sys_dictionary_api::add))
        .route("/dictionary/del", post(sys_dictionary_api::del))
        .route("/dictionary/edit", post(sys_dictionary_api::edit))
        .route("/dictionary/list", post(sys_dictionary_api::list))
        .route("/dictionary/one", post(sys_dictionary_api::one))
        .route(
            "/dictionary_detail/add",
            post(sys_dictionary_detail_api::add),
        )
        .route(
            "/dictionary_detail/del",
            post(sys_dictionary_detail_api::del),
        )
        .route(
            "/dictionary_detail/edit",
            post(sys_dictionary_detail_api::edit),
        )
        .route(
            "/dictionary_detail/list",
            post(sys_dictionary_detail_api::list),
        )
        .route(
            "/dictionary_detail/one",
            post(sys_dictionary_detail_api::one),
        )
        .route("/menu/add", post(sys_menu_api::add))
        .route("/menu/del", post(sys_menu_api::del))
        .route("/menu/edit", post(sys_menu_api::edit))
        .route("/menu/list", post(sys_menu_api::list))
        .route("/menu/one", post(sys_menu_api::one))
        .route("/role/add", post(sys_role_api::add))
        .route("/role/del", post(sys_role_api::del))
        .route("/role/edit", post(sys_role_api::edit))
        .route("/role/list", post(sys_role_api::list))
        .route("/role/one", post(sys_role_api::one))
        .route("/user/add", post(sys_user_api::add))
        .route("/user/del", post(sys_user_api::del))
        .route("/user/edit", post(sys_user_api::edit))
        .route("/user/list", post(sys_user_api::list))
        .route("/user/one", post(sys_user_api::one))
        .route("/user_role/add", post(sys_user_role_api::add))
        .route("/user_role/del", post(sys_user_role_api::del))
        .route("/user_role/edit", post(sys_user_role_api::edit))
        .route("/user_role/list", post(sys_user_role_api::list))
        .route("/user_role/one", post(sys_user_role_api::one))
}
