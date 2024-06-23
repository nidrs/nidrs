use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_attribute]
pub fn dto(args: TokenStream, input: TokenStream) -> TokenStream {
    let raw_input = TokenStream2::from(input.clone());

    TokenStream::from(quote! {
        #raw_input
    })
}

#[proc_macro_derive(Validator, attributes(rule))]
pub fn validate_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    // let mut validations = vec![];

    // if let Data::Struct(data) = input.data {
    //     for field in data.fields {
    //         let field_name = &field.ident;

    //         for attr in field.attrs.iter().filter(|a| a.path().is_ident("rule")) {
    //             if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
    //                 for nested_meta in meta_list.nested.iter() {
    //                     if let NestedMeta::Meta(Meta::NameValue(meta_name_value)) = nested_meta {
    //                         if meta_name_value.path.is_ident("Email") {
    //                             validations.push(quote! {
    //                                 if !self.#field_name.contains('@') {
    //                                     return Err(format!("Invalid email: {}", self.#field_name));
    //                                 }
    //                             });
    //                         } else if meta_name_value.path.is_ident("Number") {
    //                             if let Lit::Str(ref lit_str) = meta_name_value.lit {
    //                                 let number_rules: Vec<&str> = lit_str.value().split(',').collect();
    //                                 for rule in number_rules {
    //                                     if rule.trim().starts_with("max(") {
    //                                         if let Some(max_val) = rule.trim().strip_prefix("max(").and_then(|s| s.strip_suffix(')')) {
    //                                             let max_val: i32 = max_val.parse().unwrap();
    //                                             validations.push(quote! {
    //                                                 if self.#field_name > #max_val {
    //                                                     return Err(format!("{} must be less than or equal to {}", stringify!(#field_name), #max_val));
    //                                                 }
    //                                             });
    //                                         }
    //                                     } else if rule.trim().starts_with("min(") {
    //                                         if let Some(min_val) = rule.trim().strip_prefix("min(").and_then(|s| s.strip_suffix(')')) {
    //                                             let min_val: i32 = min_val.parse().unwrap();
    //                                             validations.push(quote! {
    //                                                 if self.#field_name < #min_val {
    //                                                     return Err(format!("{} must be greater than or equal to {}", stringify!(#field_name), #min_val));
    //                                                 }
    //                                             });
    //                                         }
    //                                     }
    //                                 }
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

    let expanded = quote! {
        impl nidrs::valid::validator::Validator for #name{
            fn valid(&self) -> ValidResult {
                use nidrs::valid::ruleset;
                use nidrs::valid::ruleset::*;
                ruleset::Email::default().valid(&self.name, "name", None)?;
                ruleset::Number::default().max(12).min(10).valid(&self.age, "age", None)?;
                // expr!(self.age > 10 && self.age < 100, "age must be greater than 0")?;
                return Ok(());
            }
        }
    };

    TokenStream::from(expanded)
}
