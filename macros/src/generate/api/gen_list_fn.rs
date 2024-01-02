use proc_macro2::{TokenStream};
use quote::quote;
use crate::generate::def::GenStructContext;

pub fn gen_list_fn(_context: &GenStructContext) -> TokenStream {
    // let m = Ident::new(&context.struct_name, Span::call_site());
    quote!(
        pub async fn list() -> Html<&'static str> {
            Html("<h1>user_add</h1>")
        }
    )
}