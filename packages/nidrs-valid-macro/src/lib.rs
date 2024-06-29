use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{parse::Parse, parse_macro_input, punctuated::Punctuated, Data, DeriveInput, Expr, Meta};

#[proc_macro_attribute]
pub fn dto(args: TokenStream, input: TokenStream) -> TokenStream {
    let raw_input = TokenStream2::from(input.clone());

    TokenStream::from(quote! {
        #[derive(serde::Serialize, serde::Deserialize, Debug, nidrs::valid_macro::Validator)]
        #raw_input
    })
}

#[proc_macro_derive(Validator, attributes(rule))]
pub fn validate_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let mut validations: Vec<TokenStream2> = vec![];
    // println!("data: {:?}", input.data);

    if let Data::Struct(data) = input.data {
        for field in data.fields {
            let field_name = &field.ident;

            for attr in field.attrs.iter().filter(|a| a.path().is_ident("rule")) {
                let meta = attr.meta.clone();

                if let Meta::List(meta_list) = meta {
                    let sub = meta_list.path.to_token_stream().to_string();
                    if sub == "rule" {
                        let args: TokenArgs = meta_list.parse_args().unwrap();
                        let rule = args.args.first().expect("rule is required");
                        let message = args
                            .args
                            .get(1)
                            .map(|msg| {
                                quote! {
                                    Some(#msg.to_string())
                                }
                            })
                            .unwrap_or(quote! {
                                None
                            });
                        validations.push(quote! {
                            let v = &self.#field_name;
                            #rule.valid(v, stringify!(#field_name), #message)?;
                        });
                    }
                }
            }
        }
    } else if let Data::Enum(data) = input.data {
        let mut variants = vec![];
        for variant in data.variants {
            let variant_name = &variant.ident;
            variants.push(quote! {
                #name::#variant_name(v) => v.valid()?,
            });
        }
        validations.push(quote! {
            match self {
                #(#variants)*
            }
        });
    } else {
        panic!("not supported")
    }
    let expanded = quote! {
        impl nidrs::valid::validator::Validator for #name{
            fn valid(&self) -> nidrs::valid::validator::ValidResult {
                use nidrs::valid::validator::Rule;
                use nidrs::valid::ruleset;
                use nidrs::valid::ruleset::*;
                #(#validations)*
                return Ok(());
            }

            fn example(&self) -> Vec<serde_json::Value> {
                vec![]
            }
        }
    };

    TokenStream::from(expanded)
}

#[derive(Clone, Debug)]
struct TokenArgs {
    pub args: Vec<Expr>,
}

impl Parse for TokenArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let items: Punctuated<Expr, syn::Token![,]> = Punctuated::parse_terminated(input)?;
        let mut args = Vec::new();
        items.iter().for_each(|item| {
            args.push(item.clone());
        });
        Ok(TokenArgs { args })
    }
}
