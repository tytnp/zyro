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
}

pub fn find_filed_fn(c: &GenStructContext, field_name: &str) -> bool {
    c.fields.iter().any(|(k, _)| k.to_string() == field_name)
}