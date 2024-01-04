use std::fs;
use std::fs::File;
use std::path::Path;
use std::process::Command;
use proc_macro2::TokenStream;
use quote::quote;
use crate::generate::models::{GenStructContext};
use crate::generate::api::gen_add_fn::gen_add_fn;
use crate::generate::api::gen_del_fn::gen_del_fn;
use crate::generate::api::gen_edit_fn::gen_edit_fn;
use crate::generate::api::gen_list_fn::gen_list_fn;
use crate::generate::api::gen_one_fn::gen_one_fn;
use crate::generate::utils::find_filed_fn;

pub fn gen_all_api(context: &GenStructContext) {
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

    let is_use_dyn =  match find_filed_fn(context,"created_at") {
        true=>{
            quote!( use chrono::Utc;use sea_orm::{ActiveValue,NotSet};)
        },
        false=>{
            TokenStream::default()
        },

    };

    let code = quote!(
        use crate::model::ApiRet;
        use crate::AppState;
        use anyhow::Error;
        use axum::extract::*;
        use axum::response::Html;
        #is_use_dyn
        use entity::model::prelude::*;
        use entity::model::*;
        use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel};
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