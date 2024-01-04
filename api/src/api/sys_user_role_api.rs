use crate::model::ApiRet;
use crate::AppState;
use anyhow::Error;
use axum::extract::*;
use axum::response::Html;
use entity::model::prelude::*;
use entity::model::*;
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel};
pub async fn add(
    State(state): State<AppState>,
    Json(model): Json<sys_user_role::Model>,
) -> Json<ApiRet<sys_user_role::Model>> {
    let active_model = model.into_active_model();
    let result = match active_model.insert(&state.conn).await {
        Ok(m) => ApiRet::<sys_user_role::Model>::with_data(m),
        Err(err) => ApiRet::<sys_user_role::Model>::with_error(Error::from(err)),
    };
    Json(result)
}
pub async fn del(
    State(state): State<AppState>,
    Json(model): Json<sys_user_role::Model>,
) -> Json<ApiRet<sys_user_role::Model>> {
    let result = match SysUserRole::find_by_id((model.user_id, model.role_id))
        .one(&state.conn)
        .await
    {
        Ok(Some(res)) => match SysUserRole::delete_by_id((model.user_id, model.role_id))
            .exec(&state.conn)
            .await
        {
            Ok(_) => ApiRet::<sys_user_role::Model>::with_data(res),
            Err(err) => ApiRet::<sys_user_role::Model>::with_error(Error::from(err)),
        },
        Err(err) => ApiRet::<sys_user_role::Model>::with_error(Error::from(err)),
        _ => ApiRet::<sys_user_role::Model>::new(),
    };
    return Json(result);
}
pub async fn edit(
    State(state): State<AppState>,
    Json(model): Json<sys_user_role::Model>,
) -> Json<ApiRet<sys_user_role::Model>> {
    let result = match SysUserRole::find_by_id((model.user_id, model.role_id))
        .one(&state.conn)
        .await
    {
        Ok(Some(res)) => {
            let mut active_model = res.into_active_model();
            active_model
                .set_from_json(serde_json::to_value(&model).unwrap())
                .unwrap();
            match active_model.update(&state.conn).await {
                Ok(m) => ApiRet::<sys_user_role::Model>::with_data(m),
                Err(err) => ApiRet::<sys_user_role::Model>::with_error(Error::from(err)),
            }
        }
        Err(err) => ApiRet::<sys_user_role::Model>::with_error(Error::from(err)),
        _ => ApiRet::<sys_user_role::Model>::new(),
    };
    Json(result)
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
    Json(result)
}
