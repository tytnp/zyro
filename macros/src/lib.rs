mod r#macro;

use proc_macro::{TokenStream};
use quote::quote;


#[proc_macro]
pub fn register_api(input: TokenStream) -> TokenStream {
    println!("{}", input.to_string());
    let expanded = quote! {

    };
    TokenStream::from(expanded)
}