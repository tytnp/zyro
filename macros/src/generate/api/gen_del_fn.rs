use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use str_case_conv::pascal_case;
use crate::generate::models::GenStructContext;
use crate::generate::utils::get_primary_key_fields;

pub fn gen_del_fn(context: &GenStructContext) -> TokenStream {
    let m = Ident::new(&context.struct_name, Span::call_site());
    let pm = Ident::new(pascal_case(&context.struct_name).as_str(), Span::call_site());
    let pk = get_primary_key_fields(&context);
    quote!(
        pub async fn del(
            State(state): State<AppState>,
            Json(model): Json<#m::Model>,
        ) -> Json<ApiRet<#m::Model>> {
            let result = match #pm::find_by_id(#pk).one(&state.conn).await {
                Ok(Some(res)) => match #pm::delete_by_id(#pk).exec(&state.conn).await {
                    Ok(_) => ApiRet::<#m::Model>::with_data(res),
                    Err(err) => ApiRet::<#m::Model>::with_error(Error::from(err)),
                    // _ => ApiRet::<#m::Model>::new(),
                },
                Err(err) => ApiRet::<#m::Model>::with_error(Error::from(err)),
                _ => ApiRet::<#m::Model>::new(),
            };
            return Json(result);
        }
    )
}
