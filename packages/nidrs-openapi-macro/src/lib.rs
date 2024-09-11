use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{ItemFn, ItemStruct};

#[proc_macro_attribute]
pub fn api(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as ItemFn);

    let mut args: Vec<TokenStream2> = vec![];

    input.sig.inputs.iter().for_each(|arg| {
        if let syn::FnArg::Typed(pat) = arg {
            let tokens = pat.ty.to_token_stream();
            args.push(quote! {
                .merge_type::<#tokens>()
            })
        }
    });

    quote! {
        #[meta(disable_auto_json = true)]
        #[meta(nidrs::openapi::RouterIn(
            nidrs::openapi::RouterParams::default()
            #(#args)*
        ))]
        #input
    }
    .into()
}

#[proc_macro_attribute]
pub fn schema(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as ItemStruct);
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let ident = &input.ident;

    quote! {
        #[derive(nidrs::openapi::utoipa::IntoParams, nidrs::openapi::utoipa::ToSchema)]
        #input

        impl #impl_generics nidrs::openapi::ToParamDto for #ident #ty_generics #where_clause {
            fn to_param_dto(dto_type: nidrs::openapi::ParamDtoIn) -> nidrs::openapi::ParamDto {
                use nidrs::openapi::utoipa::IntoParams;
                use nidrs::openapi::utoipa::ToSchema;
                match dto_type {
                    nidrs::openapi::ParamDtoIn::Param(p) => nidrs::openapi::ParamDto::ParamList(Self::into_params(|| Some(p.clone()))),
                    nidrs::openapi::ParamDtoIn::Body => nidrs::openapi::ParamDto::BodySchema(Self::schema()),
                }
            }
        }
    }
    .into()
}
