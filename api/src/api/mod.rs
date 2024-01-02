use crate::AppState;
use axum::routing::get;
use axum::Router;
mod sys_api_api;
mod sys_dictionary_api;
mod sys_dictionary_detail_api;
mod sys_menu_api;
mod sys_role_api;
mod sys_role_menu_api;
mod sys_user_api;
mod sys_user_role_api;
pub fn register_api(app: Router<AppState>) -> Router<AppState> {
    app.route("/api/add", get(sys_api_api::add))
        .route("/api/del", get(sys_api_api::del))
        .route("/api/edit", get(sys_api_api::edit))
        .route("/api/list", get(sys_api_api::list))
        .route("/api/one", get(sys_api_api::one))
        .route("/dictionary/add", get(sys_dictionary_api::add))
        .route("/dictionary/del", get(sys_dictionary_api::del))
        .route("/dictionary/edit", get(sys_dictionary_api::edit))
        .route("/dictionary/list", get(sys_dictionary_api::list))
        .route("/dictionary/one", get(sys_dictionary_api::one))
        .route(
            "/dictionary_detail/add",
            get(sys_dictionary_detail_api::add),
        )
        .route(
            "/dictionary_detail/del",
            get(sys_dictionary_detail_api::del),
        )
        .route(
            "/dictionary_detail/edit",
            get(sys_dictionary_detail_api::edit),
        )
        .route(
            "/dictionary_detail/list",
            get(sys_dictionary_detail_api::list),
        )
        .route(
            "/dictionary_detail/one",
            get(sys_dictionary_detail_api::one),
        )
        .route("/menu/add", get(sys_menu_api::add))
        .route("/menu/del", get(sys_menu_api::del))
        .route("/menu/edit", get(sys_menu_api::edit))
        .route("/menu/list", get(sys_menu_api::list))
        .route("/menu/one", get(sys_menu_api::one))
        .route("/role/add", get(sys_role_api::add))
        .route("/role/del", get(sys_role_api::del))
        .route("/role/edit", get(sys_role_api::edit))
        .route("/role/list", get(sys_role_api::list))
        .route("/role/one", get(sys_role_api::one))
        .route("/role_menu/add", get(sys_role_menu_api::add))
        .route("/role_menu/del", get(sys_role_menu_api::del))
        .route("/role_menu/edit", get(sys_role_menu_api::edit))
        .route("/role_menu/list", get(sys_role_menu_api::list))
        .route("/role_menu/one", get(sys_role_menu_api::one))
        .route("/user/add", get(sys_user_api::add))
        .route("/user/del", get(sys_user_api::del))
        .route("/user/edit", get(sys_user_api::edit))
        .route("/user/list", get(sys_user_api::list))
        .route("/user/one", get(sys_user_api::one))
        .route("/user_role/add", get(sys_user_role_api::add))
        .route("/user_role/del", get(sys_user_role_api::del))
        .route("/user_role/edit", get(sys_user_role_api::edit))
        .route("/user_role/list", get(sys_user_role_api::list))
        .route("/user_role/one", get(sys_user_role_api::one))
}
