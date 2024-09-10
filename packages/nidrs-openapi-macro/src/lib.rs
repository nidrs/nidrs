use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{token::Struct, ItemFn, ItemStruct};

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
        #[meta(nidrs::externs::shared::block({
            nidrs::openapi::RouterParams::default()
            #(#args)*
        }))]
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
            fn to_param_dto(dto_type: nidrs::openapi::ParamDtoType) -> nidrs::openapi::ParamDto {
                use nidrs::openapi::utoipa::IntoParams;
                use nidrs::openapi::utoipa::ToSchema;
                match dto_type {
                    nidrs::openapi::ParamDtoType::Parameter(p) => nidrs::openapi::ParamDto::Parameters(Self::into_params(|| Some(p.clone()))),
                    nidrs::openapi::ParamDtoType::RequestBody => nidrs::openapi::ParamDto::RequestBodies(Self::schema()),
                }
            }
        }
    }
    .into()
}
