use std::collections::HashSet;

use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::ItemStruct;
use syn_args::def;

/// Merge derives from the function with the default derives
pub fn merge_derives(func: &ItemStruct, default_derives: &[&str]) -> Vec<TokenStream> {
    let mut derives = HashSet::new();
    default_derives.iter().for_each(|derive| {
        derives.insert(derive.to_string());
    });

    func.attrs.iter().for_each(|attr| {
        let ident = attr.meta.path();
        let ident = ident.to_token_stream().to_string();
        if ident.contains("derive") {
            let ext_derives = syn::parse2::<syn_args::SynArgs>(attr.meta.to_token_stream()).unwrap().arguments::<syn_args::Arguments>().unwrap();
            let ext_derives: def::Extends<def::Expr> = ext_derives.try_into().unwrap();

            ext_derives.iter().for_each(|derive| {
                let t = derive.to_path_name().unwrap();
                derives.remove(&t);
            });
        }
    });

    let derives_tokens = derives
        .iter()
        .map(|derive| {
            let derive_ident = syn::Ident::new(derive, Span::call_site());
            quote! {
                #[derive(#derive_ident)]
            }
        })
        .collect::<Vec<TokenStream>>();
    derives_tokens
}
