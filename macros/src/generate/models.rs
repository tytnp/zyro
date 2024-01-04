use std::collections::HashMap;
use darling::FromMeta;
use proc_macro2::Ident;
use syn::Type;

#[derive(Debug, FromMeta)]
pub struct SeaOrmArgs {
    pub table_name: String,
}

#[derive(Debug)]
pub struct GenStructContext {
    pub source_path: String,
    pub target_path: String,
    pub struct_name: String,
    pub fields: HashMap<Ident, Type>,
    pub primary_keys: Vec<String>
}

