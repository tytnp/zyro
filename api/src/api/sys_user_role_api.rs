use crate::model::ApiRet;
use crate::AppState;
use axum::extract::*;
use axum::response::Html;
use entity::model::prelude::*;
use entity::model::*;
use sea_orm::{EntityTrait};
use anyhow::Error;

pub async fn add(
    State(state): State<AppState>,
    Json(model): Json<sys_user_role::Model>,
) -> Json<sys_user_role::Model> {
    Json(model)
}

pub async fn del(
    State(state): State<AppState>,
    Json(model): Json<sys_user_role::Model>,
) -> Json<sys_user_role::Model> {
    Json(model)
}

pub async fn edit(
    State(state): State<AppState>,
    Json(model): Json<sys_user_role::Model>,
) -> Json<sys_user_role::Model> {
    Json(model)
}

pub async fn list() -> Html<&'static str> {
    Html("<h1>user_add</h1>")
}

pub async fn one(
    State(state): State<AppState>,
    Json(model): Json<sys_user_role::Model>,
) -> Json<ApiRet<sys_user_role::Model>> {
    let result = match SysUserRole::find_by_id((model.user_id, model.role_id))
        .one(&state.conn)
        .await
    {
        Ok(Some(m)) => ApiRet::<sys_user_role::Model>::with_data(m),
        Err(err) => ApiRet::<sys_user_role::Model>::with_error(Error::from(err)),
        _ => ApiRet::<sys_user_role::Model>::new(),
    };
    let a = Json(result);
    return a;
}
