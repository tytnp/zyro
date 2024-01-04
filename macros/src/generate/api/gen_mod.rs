use std::fs;
use std::fs::File;
use std::ops::Add;
use std::path::Path;
use std::process::Command;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use crate::generate::models::{ GenStructContext};

pub fn gen_mod(target_path: &str, contexts: &Vec<GenStructContext>) {
    let target_file = Path::new(&target_path).join("mod.rs");
    if !target_file.exists() {
        if let Some(parent) = target_file.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).unwrap();
            }
        }
        File::create(&target_file).unwrap();
    }
    let mods: Vec<TokenStream> = contexts
        .iter()
        .map(|c| {
            let m = Ident::new(&*c.struct_name.clone().add("_api"), Span::call_site());
            quote!(
                mod #m;
            )
        })
        .collect();
    let format = |name: &str| -> String {
        let index = name.find('_').unwrap_or_else(|| name.len());
        name[index + 1..].to_string()
    };

    let routes: Vec<TokenStream> = contexts
        .iter()
        .map(|c| {
            let m = Ident::new(&*c.struct_name.clone().add("_api"), Span::call_site());
            let add_path = format!("/{}/add", format(&c.struct_name));
            let del_path = format!("/{}/del", format(&c.struct_name));
            let edit_path = format!("/{}/edit", format(&c.struct_name));
            let list_path = format!("/{}/list", format(&c.struct_name));
            let one_path = format!("/{}/one", format(&c.struct_name));
            quote!(
                .route(#add_path, post(#m::add))
                .route(#del_path, post(#m::del))
                .route(#edit_path, post(#m::edit))
                .route(#list_path, post(#m::list))
                .route(#one_path, post(#m::one))
            )
        })
        .collect();
    let code = quote!(
        use axum::Router;
        use axum::routing::*;
        use crate::AppState;
        #(#mods)*
        pub fn register_api(app: Router<AppState>) -> Router<AppState> {
            app
             #(#routes)*

        }
    );
    fs::write(&target_file, code.to_string()).unwrap();
    Command::new("rustfmt")
        .arg("--edition=2021")
        .arg(&target_file)
        .status()
        .unwrap();
    ()
}