use std::fs;
use std::fs::File;
use std::path::Path;
use std::process::Command;
use quote::quote;
use crate::generate::def::{find_filed_fn, GenStructContext};
use crate::generate::api::gen_add_fn::gen_add_fn;
use crate::generate::api::gen_del_fn::gen_del_fn;
use crate::generate::api::gen_edit_fn::gen_edit_fn;
use crate::generate::api::gen_list_fn::gen_list_fn;
use crate::generate::api::gen_one_fn::gen_one_fn;

pub fn gen_all_api(context: &GenStructContext) {
    // if !find_filed_fn(context,"created_at") {
    //     return ()
    // }
    if !Path::new(&context.target_path).exists() {
        if let Some(parent) = Path::new(&context.target_path).parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).unwrap();
            }
        }
        File::create(&context.target_path).unwrap();
    }
    let add = gen_add_fn(&context);
    let del = gen_del_fn(&context);
    let edit = gen_edit_fn(&context);
    let list = gen_list_fn(&context);
    let one = gen_one_fn(&context);
    let code = quote!(
        use crate::AppState;
        use axum::extract::*;
        use axum::response::Html;
        use chrono::Utc;
        use entity::model::prelude::*;
        use entity::model::*;
        use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait, IntoActiveModel, NotSet};
        #add
        #del
        #edit
        #list
        #one
    );
    fs::write(&context.target_path, code.to_string()).unwrap();
    Command::new("rustfmt")
        .arg("--edition=2021")
        .arg(&context.target_path)
        .status()
        .unwrap();
    ()
}