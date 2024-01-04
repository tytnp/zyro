use crate::generate::models::GenStructContext;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote};
use crate::generate::utils::find_filed_fn;

pub fn gen_add_fn(context: &GenStructContext) -> TokenStream {
    let m = Ident::new(&context.struct_name, Span::call_site());
    let dyn_content =  get_add_dyn_content(&context);
    let is_mut =  match find_filed_fn(context,"created_at") {
        true=>{
            quote!( mut)
        },
        false=>{
            TokenStream::default()
        },

    };
    quote!(
        pub async fn add(
            State(state): State<AppState>,
            Json(model): Json<#m::Model>,
        ) -> Json<ApiRet<#m::Model>> {
            let #is_mut active_model = model.into_active_model();
            #dyn_content
            let result = match active_model.insert(&state.conn).await{
                Ok(m) => ApiRet::<#m::Model>::with_data(m),
                Err(err) => ApiRet::<#m::Model>::with_error(Error::from(err)),
                // _ => ApiRet::<#m::Model>::new(),
            };
            Json(result)
        }
    )
}

pub fn get_add_dyn_content(context: &GenStructContext) -> TokenStream {
    let pk = if context.primary_keys.len() == 1 {
        let one_key = Ident::new(context.primary_keys.get(0).unwrap(), Span::call_site());
        quote!(
             active_model.#one_key = NotSet;
        )
    }else{
        TokenStream::default()
    };

    let time = if find_filed_fn(context, "created_at") {
        quote!(
            active_model.created_at = ActiveValue::Set(Option::from(Utc::now()));
        )
    }else {
        TokenStream::default()
    };
    quote!(
        #pk
        #time
    )
}
