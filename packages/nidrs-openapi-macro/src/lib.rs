use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{ItemEnum, ItemFn, ItemStruct};
use syn_args::{def, derive::ArgsParse, ArgsParse};

static PARAM_NAMES: [&str; 7] = ["Body", "Path", "Query", "Header", "Cookie", "Form", "Json"];

#[proc_macro_attribute]
pub fn api(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as ItemFn);

    let mut router_in: Vec<TokenStream2> = vec![];

    input.sig.inputs.iter().for_each(|arg| {
        if let syn::FnArg::Typed(pat) = arg {
            let var_type = pat.ty.to_token_stream();
            if let syn::Pat::Ident(pat_ident) = &*pat.pat {
                let var_type_str = var_type.to_string();
                if PARAM_NAMES.iter().any(|&param| var_type_str.starts_with(param)) {
                    let name = pat_ident.ident.to_string();
                    router_in.push(quote! {
                        .comb::<#var_type>(#name)
                    });
                }
            }
        }
    });

    let mut router_out: Vec<TokenStream2> = vec![];

    if let syn::ReturnType::Type(_, ty) = &input.sig.output {
        router_out.push(quote! {
            .comb::<#ty>("")
        });
    }

    quote! {
        // #[meta(disable_auto_json = true)]
        #[meta(
            nidrs::openapi::RouterIn(
                nidrs::openapi::RouterParams::default()
                #(#router_in)*,
            ),
            nidrs::openapi::RouterOut(
                nidrs::openapi::RouterParams::default()
                #(#router_out)*,
            )
        )]
        #input
    }
    .into()
}

#[syn_args::derive::declare(def::Expr)]
#[syn_args::derive::proc_attribute]
pub fn api_security(args: Args, input: TokenStream) -> TokenStream {
    let security_key = if let Args::F1(def::Expr(syn::Expr::Path(expr_path))) = args {
        expr_path.to_token_stream().to_string()
    } else {
        panic!("[nidrs-openapi-macro.api_security] Invalid input type");
    };

    let input = syn::parse_macro_input!(input as ItemFn);
    quote! {
        #[nidrs::meta(
            nidrs::openapi::RouterSecurity(#security_key.to_string())
        )]
        #input
    }
    .into()
}

#[syn_args::derive::declare()]
#[syn_args::derive::declare(def::Expr)]
#[syn_args::derive::proc_attribute]
pub fn schema(args: Args, input: TokenStream) -> TokenStream {
    let ident_arg: syn::Ident = {
        if let Args::F2(def::Expr(syn::Expr::Path(expr_path))) = args {
            if let Some(ident) = expr_path.path.get_ident() {
                ident.clone()
            } else {
                syn::Ident::new("ToSchema", proc_macro2::Span::call_site())
            }
        } else {
            syn::Ident::new("ToSchema", proc_macro2::Span::call_site())
        }
    };

    if let Ok(input_struct) = syn::parse::<ItemStruct>(input.clone()) {
        // Struct implementation
        let (impl_generics, ty_generics, where_clause) = input_struct.generics.split_for_impl();
        let ident = &input_struct.ident;

        let impl_param_dto_in = quote! {
            let ref_schema: RefOr<Schema> = utoipa::schema!(Self).into();
            let mut schemas: Vec<(String, RefOr<Schema>)> = vec![
                (
                    <Self as utoipa::ToSchema>::name().to_string(),
                    utoipa::schema!(#[inline] Self).into(),
                )
            ];

            <Self as utoipa::ToSchema>::schemas(&mut schemas);
            nidrs::openapi::ParamDto::BodySchema((
                ref_schema,
                schemas,
            ))
        };

        let (impl_derive, impl_match) = if ident_arg == "IntoParams" {
            (
                quote! {
                    #[derive(nidrs::openapi::utoipa::ToSchema, nidrs::openapi::utoipa::IntoParams)]
                },
                quote! {
                    match dto_type {
                        nidrs::openapi::ParamDtoIn::Body => {
                            #impl_param_dto_in
                        },
                        nidrs::openapi::ParamDtoIn::Param(p) => nidrs::openapi::ParamDto::ParamList(Self::into_params(|| Some(p.clone()))),
                    }
                },
            )
        } else {
            (
                quote! {
                    #[derive(nidrs::openapi::utoipa::ToSchema)]
                },
                quote! {
                    match dto_type {
                        nidrs::openapi::ParamDtoIn::Body => {
                            #impl_param_dto_in
                        },
                        _ => nidrs::openapi::ParamDto::None,
                    }
                },
            )
        };

        quote! {
            #impl_derive
            #input_struct

            impl #impl_generics nidrs::openapi::ToParamDto for #ident #ty_generics #where_clause {
                fn to_param_dto(dto_type: nidrs::openapi::ParamDtoIn) -> nidrs::openapi::ParamDto {
                    use nidrs::openapi::utoipa::IntoParams;
                    use nidrs::openapi::utoipa::ToSchema;
                    use nidrs::openapi::utoipa::openapi::Schema;
                    use nidrs::openapi::utoipa::openapi::RefOr;
                    use nidrs::openapi::utoipa;

                    #impl_match
                }
            }
        }
        .into()
    } else if let Ok(input_enum) = syn::parse::<ItemEnum>(input.clone()) {
        // Enum implementation
        let (impl_generics, ty_generics, where_clause) = input_enum.generics.split_for_impl();
        let ident = &input_enum.ident;

        quote! {
            #[derive(nidrs::openapi::utoipa::ToSchema)]
            #input_enum

            impl #impl_generics nidrs::openapi::ToParamDto for #ident #ty_generics #where_clause {
                fn to_param_dto(dto_type: nidrs::openapi::ParamDtoIn) -> nidrs::openapi::ParamDto {
                    use nidrs::openapi::utoipa::ToSchema;
                    use nidrs::openapi::utoipa::openapi::Schema;
                    use nidrs::openapi::utoipa::openapi::RefOr;
                    use nidrs::openapi::utoipa;

                    let ref_schema: RefOr<Schema> = utoipa::schema!(Self).into();
                    let mut schemas: Vec<(String, RefOr<Schema>)> = vec![
                        (
                            <Self as utoipa::ToSchema>::name().to_string(),
                            utoipa::schema!(#[inline] Self).into(),
                        )
                    ];

                    <Self as utoipa::ToSchema>::schemas(&mut schemas);

                    match dto_type {
                        nidrs::openapi::ParamDtoIn::Body => nidrs::openapi::ParamDto::BodySchema((
                            ref_schema,
                            schemas,
                        )),
                        _ => nidrs::openapi::ParamDto::None,
                    }
                }
            }
        }
        .into()
    } else {
        panic!("[nidrs-openapi-macro.schema] Invalid input type");
    }
}
