use crate::model::ApiRet;
use crate::AppState;
use anyhow::Error;
use axum::extract::*;
use axum::response::Html;
use chrono::Utc;
use entity::model::prelude::*;
use entity::model::*;
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel};
use sea_orm::{ActiveValue, NotSet};
pub async fn add(
    State(state): State<AppState>,
    Json(model): Json<sys_dictionary::Model>,
) -> Json<ApiRet<sys_dictionary::Model>> {
    let mut active_model = model.into_active_model();
    active_model.id = NotSet;
    active_model.created_at = ActiveValue::Set(Option::from(Utc::now()));
    let result = match active_model.insert(&state.conn).await {
        Ok(m) => ApiRet::<sys_dictionary::Model>::with_data(m),
        Err(err) => ApiRet::<sys_dictionary::Model>::with_error(Error::from(err)),
    };
    Json(result)
}
pub async fn del(
    State(state): State<AppState>,
    Json(model): Json<sys_dictionary::Model>,
) -> Json<ApiRet<sys_dictionary::Model>> {
    let result = match SysDictionary::find_by_id(model.id).one(&state.conn).await {
        Ok(Some(res)) => match SysDictionary::delete_by_id(model.id)
            .exec(&state.conn)
            .await
        {
            Ok(_) => ApiRet::<sys_dictionary::Model>::with_data(res),
            Err(err) => ApiRet::<sys_dictionary::Model>::with_error(Error::from(err)),
        },
        Err(err) => ApiRet::<sys_dictionary::Model>::with_error(Error::from(err)),
        _ => ApiRet::<sys_dictionary::Model>::new(),
    };
    return Json(result);
}
pub async fn edit(
    State(state): State<AppState>,
    Json(model): Json<sys_dictionary::Model>,
) -> Json<ApiRet<sys_dictionary::Model>> {
    let result = match SysDictionary::find_by_id(model.id).one(&state.conn).await {
        Ok(Some(res)) => {
            let mut active_model = res.into_active_model();
            let created_at = active_model.created_at.clone();
            active_model
                .set_from_json(serde_json::to_value(&model).unwrap())
                .unwrap();
            active_model.created_at = created_at;
            active_model.updated_at = ActiveValue::Set(Option::from(Utc::now()));
            match active_model.update(&state.conn).await {
                Ok(m) => ApiRet::<sys_dictionary::Model>::with_data(m),
                Err(err) => ApiRet::<sys_dictionary::Model>::with_error(Error::from(err)),
            }
        }
        Err(err) => ApiRet::<sys_dictionary::Model>::with_error(Error::from(err)),
        _ => ApiRet::<sys_dictionary::Model>::new(),
    };
    Json(result)
}
pub async fn list() -> Html<&'static str> {
    Html("<h1>user_add</h1>")
}
pub async fn one(
    State(state): State<AppState>,
    Json(model): Json<sys_dictionary::Model>,
) -> Json<ApiRet<sys_dictionary::Model>> {
    let result = match SysDictionary::find_by_id(model.id).one(&state.conn).await {
        Ok(Some(m)) => ApiRet::<sys_dictionary::Model>::with_data(m),
        Err(err) => ApiRet::<sys_dictionary::Model>::with_error(Error::from(err)),
        _ => ApiRet::<sys_dictionary::Model>::new(),
    };
    Json(result)
}
