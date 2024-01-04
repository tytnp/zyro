use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use str_case_conv::pascal_case;
use crate::generate::models::GenStructContext;
use crate::generate::utils::{find_filed_fn, get_primary_key_fields};

pub fn gen_edit_fn(context: &GenStructContext) -> TokenStream {
    let pm = Ident::new(pascal_case(&context.struct_name).as_str(), Span::call_site());
    let m = Ident::new(&context.struct_name, Span::call_site());
    let pk = get_primary_key_fields(&context);
    let dyn_content = get_edit_dyn_content(&context);
    quote!(
        // pub async fn edit(
        //     State(state): State<AppState>,
        //     Json(model): Json<#m::Model>,
        // ) -> Json<#m::Model> {
        //     let mut active_model = #pm::find_by_id(model.id)
        //         .one(&state.conn)
        //         .await
        //         .unwrap()
        //         .unwrap()
        //         .into_active_model();
        //     let created_at = active_model.created_at.clone();
        //     active_model
        //         .set_from_json(serde_json::to_value(&model).unwrap())
        //         .unwrap();
        //     active_model.created_at = created_at;
        //     active_model.updated_at = ActiveValue::Set(Option::from(Utc::now()));
        //     Json(active_model.update(&state.conn).await.unwrap())
        // }

        pub async fn edit(State(state): State<AppState>, Json(model): Json<#m::Model>) -> Json<ApiRet<#m::Model>> {
            let result = match #pm::find_by_id(#pk)
                .one(&state.conn)
                .await
            {
                Ok(Some(res)) => {
                    let mut active_model = res.into_active_model();
                    // let created_at = active_model.created_at.clone();
                    // active_model
                    //     .set_from_json(serde_json::to_value(&model).unwrap())
                    //     .unwrap();
                    // active_model.created_at = created_at;
                    // active_model.updated_at = ActiveValue::Set(Option::from(Utc::now()));
                    #dyn_content
                    match active_model.update(&state.conn).await {
                        Ok(m) => ApiRet::<#m::Model>::with_data(m),
                        Err(err) => ApiRet::<#m::Model>::with_error(Error::from(err)),
                        // _ => ApiRet::<#m::Model>::new(),
                    }
                },
                Err(err) => ApiRet::<#m::Model>::with_error(Error::from(err)),
                _ => ApiRet::<#m::Model>::new(),
            };
            Json(result)
        }
    )
}

pub fn get_edit_dyn_content(context: &GenStructContext) -> TokenStream {
    let created_at_clone = if find_filed_fn(context, "created_at") {
        quote!(
            let created_at = active_model.created_at.clone();
        )
    }else {
        TokenStream::default()
    };
    let created_at = if find_filed_fn(context, "created_at") {
        quote!(
            active_model.created_at = created_at;
        )
    }else {
        TokenStream::default()
    };
    let updated_at = if find_filed_fn(context, "updated_at") {
        quote!(
            active_model.updated_at = ActiveValue::Set(Option::from(Utc::now()));
        )
    }else {
        TokenStream::default()
    };
    quote!(
        #created_at_clone
        active_model.set_from_json(serde_json::to_value(&model).unwrap()).unwrap();
        #created_at
        #updated_at
    )
}