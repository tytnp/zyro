use crate::generate::api::gen_all_api::gen_all_api;
use crate::generate::api::gen_mod::gen_mod;
use crate::generate::def::*;
use darling::FromMeta;
use std::collections::HashMap;
use std::fs;
use std::ops::Add;
use std::path::Path;
use syn::Item;

mod gen_add_fn;
mod gen_all_api;
mod gen_del_fn;
mod gen_edit_fn;
mod gen_list_fn;
mod gen_mod;
mod gen_one_fn;


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
                for item in ast.items {
                    if let Item::Struct(struct_item) = item {
                        for struct_attr in struct_item.attrs {
                            if let Ok(args) = SeaOrmArgs::from_meta(&struct_attr.meta) {
                                let mut filed_map = HashMap::new();
                                for f in &struct_item.fields {
                                    filed_map.insert(f.clone().ident.unwrap(), f.clone().ty);
                                }
                                contexts.push(GenStructContext {
                                    source_path: path.to_str().unwrap().to_string(),
                                    target_path: Path::new(target_path)
                                        .join(args.table_name.clone().add("_api.rs"))
                                        .to_str()
                                        .unwrap()
                                        .to_string(),
                                    struct_name: args.table_name,
                                    fields: filed_map.clone(),
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
