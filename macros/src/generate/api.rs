use crate::generate::def;
use crate::generate::def::GenStructContext;
use darling::FromMeta;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::fs;
use std::fs::File;
use std::ops::Add;
use std::path::Path;
use std::process::Command;
use syn::Item;

#[test]
pub fn gen_api_start() {
    let source = "D:\\codebase\\ideaProjects\\zyro\\entity\\src\\model";
    let target = "D:\\codebase\\ideaProjects\\zyro\\api\\src\\api";
    let contexts = load_struct_context(&source, &target);
    mod_gen(&target, &contexts);
    for context in contexts {
        api_gen(&context);
    }
}

fn load_struct_context(source_path: &str, target_path: &str) -> Vec<GenStructContext> {
    let mut contexts = Vec::new();
    for file in fs::read_dir(source_path).unwrap() {
        if let path = file.unwrap().path() {
            if path.is_file() {
                let context = fs::read_to_string(&path).unwrap();
                let ast = syn::parse_file(&context).unwrap();
                for item in ast.items {
                    if let Item::Struct(struct_item) = item {
                        for struct_attr in struct_item.attrs {
                            if let Ok(args) = def::SeaOrmArgs::from_meta(&struct_attr.meta) {
                                contexts.push(GenStructContext {
                                    source_path: path.to_str().unwrap().to_string(),
                                    target_path: Path::new(target_path)
                                        .join(args.table_name.clone().add("_api.rs"))
                                        .to_str()
                                        .unwrap()
                                        .to_string(),
                                    struct_name: args.table_name,
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

fn mod_gen(target_path: &str, contexts: &Vec<GenStructContext>) {
    let mut target_content = String::new();
    let target_file = Path::new(&target_path).join("mod.rs");
    if target_file.exists() {
        target_content = fs::read_to_string(&target_file).unwrap();
    } else {
        if let Some(parent) = target_file.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).unwrap();
            }
        }
        File::create(&target_file).unwrap();
        target_content = fs::read_to_string(&target_file).unwrap();
    }
    let mods:Vec<TokenStream> = contexts.iter().map(|c|{
        let m = Ident::new(&*c.struct_name.clone().add("_api"), Span::call_site());
        quote!(
            mod #m;
        )
    }).collect();

    let format = |name: &str| -> String {
        let index = name.find('_').unwrap_or_else(|| name.len());
        name[index + 1..].to_string()
    };

    let routes:Vec<TokenStream> = contexts.iter().map(|c|{
        let m = Ident::new(&*c.struct_name.clone().add("_api"), Span::call_site());
        let add_path = format!("/{}/add", format(&c.struct_name)) ;
        let del_path = format!("/{}/del", format(&c.struct_name)) ;
        let edit_path = format!("/{}/edit", format(&c.struct_name)) ;
        let list_path = format!("/{}/list", format(&c.struct_name)) ;
        let one_path = format!("/{}/one", format(&c.struct_name)) ;
        //let c = Ident::new(format(&c.struct_name).as_str(), Span::call_site());
        //let route_str = format!("/{}/add", format(&c.struct_name));
        quote!(
            .route(#add_path, get(#m::add))
            .route(#del_path, get(#m::del))
            .route(#edit_path, get(#m::edit))
            .route(#list_path, get(#m::list))
            .route(#one_path, get(#m::one))
        )
    }).collect();

    let code = quote!(
        use axum::Router;
        use axum::routing::get;
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

fn api_gen(context: &GenStructContext) {
    // let mut source_content = String::new();
    let mut target_content = String::new();
    if Path::new(&context.target_path).exists() {
        target_content = fs::read_to_string(&context.target_path).unwrap();
    } else {
        if let Some(parent) = Path::new(&context.target_path).parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).unwrap();
            }
        }
        File::create(&context.target_path).unwrap();
        target_content = fs::read_to_string(&context.target_path).unwrap();
    }
    let add = add_fn_gen();
    let del = del_fn_gen();
    let edit = edit_fn_gen();
    let list = list_fn_gen();
    let one = one_fn_gen();
    let code = quote!(
        use axum::response::Html;
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

fn add_fn_gen() -> TokenStream {
    quote!(
        pub async fn add() -> Html<&'static str> {
            Html("<h1>user_add</h1>")
        }
    )
}

fn del_fn_gen() -> TokenStream {
    quote!(
        pub async fn del() -> Html<&'static str> {
            Html("<h1>user_add</h1>")
        }
    )
}

fn edit_fn_gen() -> TokenStream {
    quote!(
        pub async fn edit() -> Html<&'static str> {
            Html("<h1>user_add</h1>")
        }
    )
}

fn list_fn_gen() -> TokenStream {
    quote!(
        pub async fn list() -> Html<&'static str> {
            Html("<h1>user_add</h1>")
        }
    )
}

fn one_fn_gen() -> TokenStream {
    quote!(
        pub async fn one() -> Html<&'static str> {
            Html("<h1>user_add</h1>")
        }
    )
}
