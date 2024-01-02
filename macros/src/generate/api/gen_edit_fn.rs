use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use str_case_conv::pascal_case;
use crate::generate::def::GenStructContext;

pub fn gen_edit_fn(context: &GenStructContext) -> TokenStream {
    let pm = Ident::new(pascal_case(&context.struct_name).as_str(), Span::call_site());
    let m = Ident::new(&context.struct_name, Span::call_site());
    quote!(
        pub async fn edit(
            State(state): State<AppState>,
            Json(model): Json<#m::Model>,
        ) -> Json<#m::Model> {
            let mut active_model = #pm::find_by_id(model.id)
                .one(&state.conn)
                .await
                .unwrap()
                .unwrap()
                .into_active_model();
            let created_at = active_model.created_at.clone();
            active_model
                .set_from_json(serde_json::to_value(&model).unwrap())
                .unwrap();
            active_model.created_at = created_at;
            active_model.updated_at = ActiveValue::Set(Option::from(Utc::now()));
            Json(active_model.update(&state.conn).await.unwrap())
        }
    )
}