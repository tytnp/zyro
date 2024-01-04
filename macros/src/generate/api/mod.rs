use crate::generate::models::*;
use darling::FromMeta;
use std::collections::HashMap;
use std::fs;
use std::ops::Add;
use std::path::Path;
use proc_macro2::TokenTree::{Group, Ident};
use quote::ToTokens;
use syn::Item;
use crate::generate::api::gen_all_api::gen_all_api;
use crate::generate::api::gen_mod::gen_mod;

mod gen_add_fn;
mod gen_all_api;
mod gen_del_fn;
mod gen_edit_fn;
mod gen_list_fn;
mod gen_mod;
mod gen_one_fn;


#[test]
pub fn gen_start() {
    let source = "D:\\codebase\\ideaProjects\\zyro\\entity\\src\\model";
    let target = "D:\\codebase\\ideaProjects\\zyro\\api\\src\\api";
    let contexts = load_struct_context(&source, &target);


    gen_mod(&target, &contexts);
    for context in contexts {
        gen_all_api(&context);
    }
}

fn load_struct_context(source_path: &str, target_path: &str) -> Vec<GenStructContext> {
    let mut contexts = Vec::new();
    for file in fs::read_dir(source_path).unwrap() {
        if let Ok(file) = file {
            let path = file.path();
            if path.is_file() {
                let context = fs::read_to_string(&path).unwrap();
                let ast = syn::parse_file(&context).unwrap();
                for item in &ast.items {
                    if let Item::Struct(struct_item) = &item {
                        for struct_attr in &struct_item.attrs {
                            if let Ok(args) = SeaOrmArgs::from_meta(&struct_attr.meta) {
                                let mut filed_map = HashMap::new();
                                let mut primary_keys = Vec::new();
                                // find primary key and save fields
                                for field in &struct_item.fields {
                                    //save fields
                                    filed_map.insert(field.clone().ident.unwrap(), field.clone().ty);
                                    //find primary key
                                    let is_primary_key  = field.attrs.iter().any(|field_attr| {
                                        for tt in field_attr.meta.to_token_stream().into_iter() {
                                            if let Group(g) = tt {
                                                for tt in g.stream().into_iter() {
                                                    if let Ident (abc) = tt {
                                                        if &abc.to_string() == "primary_key"{
                                                            return true
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        return false
                                    });
                                    if is_primary_key {
                                        primary_keys.push(field.clone().ident.unwrap().to_string())
                                    }

                                }
                                // save ast struct
                                contexts.push(GenStructContext {
                                    source_path: path.to_str().unwrap().to_string(),
                                    target_path: Path::new(target_path)
                                        .join(args.table_name.clone().add("_api.rs"))
                                        .to_str()
                                        .unwrap()
                                        .to_string(),
                                    struct_name: args.table_name,
                                    fields: filed_map.clone(),
                                    primary_keys
                                })
                            }
                        }
                    }
                }
            }
        }
    }
    contexts
}
