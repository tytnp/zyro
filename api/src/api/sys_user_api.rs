use crate::AppState;
use axum::extract::*;
use axum::response::Html;
use chrono::Utc;
use entity::model::prelude::*;
use entity::model::*;
use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait, IntoActiveModel, NotSet};
pub async fn add(
    State(state): State<AppState>,
    Json(model): Json<sys_user::Model>,
) -> Json<sys_user::Model> {
    let mut active_model = model.into_active_model();
    active_model.id = NotSet;
    active_model.created_at = ActiveValue::Set(Option::from(Utc::now()));
    Json(active_model.insert(&state.conn).await.unwrap())
}
pub async fn del(
    State(state): State<AppState>,
    Json(model): Json<sys_user::Model>,
) -> Json<sys_user::Model> {
    if let Ok(Some(res)) = SysUser::find_by_id(model.id).one(&state.conn).await {
        SysUser::delete_by_id(res.id)
            .exec(&state.conn)
            .await
            .unwrap();
        return Json(res);
    }
    return Json(model);
}
pub async fn edit(
    State(state): State<AppState>,
    Json(model): Json<sys_user::Model>,
) -> Json<sys_user::Model> {
    let mut active_model = SysUser::find_by_id(model.id)
        .one(&state.conn)
        .await
        .unwrap()
        .unwrap()
        .into_active_model();
    let created_at = active_model.created_at.clone();
    active_model
        .set_from_json(serde_json::to_value(&model).unwrap())
        .unwrap();
    active_model.created_at = created_at;
    active_model.updated_at = ActiveValue::Set(Option::from(Utc::now()));
    Json(active_model.update(&state.conn).await.unwrap())
}
pub async fn list() -> Html<&'static str> {
    Html("<h1>user_add</h1>")
}
pub async fn one(
    State(state): State<AppState>,
    Json(model): Json<sys_user::Model>,
) -> Json<sys_user::Model> {
    Json(
        SysUser::find_by_id(model.id)
            .one(&state.conn)
            .await
            .unwrap()
            .unwrap(),
    )
}
