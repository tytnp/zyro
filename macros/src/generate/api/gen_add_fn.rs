use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use crate::generate::def::GenStructContext;

pub fn gen_add_fn(context: &GenStructContext) -> TokenStream {
    let m = Ident::new(&context.struct_name, Span::call_site());
    quote!(
        pub async fn add(
            State(state): State<AppState>,
            Json(model): Json<#m::Model>,
        ) -> Json<#m::Model> {
            let mut active_model = model.into_active_model();
            active_model.id = NotSet;
            active_model.created_at = ActiveValue::Set(Option::from(Utc::now()));
            Json(active_model.insert(&state.conn).await.unwrap())
        }
    )
}