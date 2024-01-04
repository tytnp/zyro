use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::generate::models::GenStructContext;

// find struct is exist field
pub fn find_filed_fn(c: &GenStructContext, field_name: &str) -> bool {
    c.fields.iter().any(|(k, _)| k.to_string() == field_name)
}

// get struct query primary key
pub fn get_primary_key_fields(context: &GenStructContext) -> TokenStream {
    let mut field_vec = Vec::new();
    for name in &context.primary_keys {
        let pm = Ident::new(name, Span::call_site());
        field_vec.push(pm);
    }
    let one_key = field_vec.get(0);
    match field_vec.len() {
        0 => TokenStream::default(),
        1 => {
            quote!(
                model.#one_key
            )
        }
        _ => {
            quote!(
                (#(model.#field_vec),*)
            )
        }
    }
}

