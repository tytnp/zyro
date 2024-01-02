use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use str_case_conv::pascal_case;
use crate::generate::def::GenStructContext;

pub fn gen_del_fn(context: &GenStructContext) -> TokenStream {
    let m = Ident::new(&context.struct_name, Span::call_site());
    let pm = Ident::new(pascal_case(&context.struct_name).as_str(), Span::call_site());
    quote!(
        pub async fn del(
            State(state): State<AppState>,
            Json(model): Json<#m::Model>,
        ) -> Json<#m::Model> {
            if let Ok(Some(res)) = #pm::find_by_id(model.id).one(&state.conn).await {
                #pm::delete_by_id(res.id)
                    .exec(&state.conn)
                    .await
                    .unwrap();
                return Json(res);
            }
            return Json(model);
        }
    )
}
