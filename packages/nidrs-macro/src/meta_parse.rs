use std::{str::FromStr, sync::Mutex};

use once_cell::sync::Lazy;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{parse_str, Expr};

use crate::MetaArgs;

static MERGE_META: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(vec![]));

pub fn clear() {
    MERGE_META.lock().unwrap().clear();
}

pub fn collect(args: MetaArgs) {
    let meta_tokens = args
        .kv
        .iter()
        .map(|(key, value)| {
            // value parse expr
            let exp = parse_str::<Expr>(&value).unwrap();
            // println!("// meta {} {} {:?}", key, value, exp);

            let v = match exp {
                Expr::Array(arr) => {
                    // arr to vec
                    let arr = arr.elems.iter().map(|elem| elem.to_owned()).collect::<Vec<Expr>>();
                    quote! {
                        Vec::from([#(#arr),*])
                    }
                }
                _ => exp.to_token_stream(),
            };

            quote! {
                meta.set(#key.to_string(), #v);
            }
        })
        .collect::<Vec<TokenStream2>>();
    let meta_tokens = TokenStream2::from(quote! {
        #(#meta_tokens)*
    });

    MERGE_META.lock().unwrap().push(meta_tokens.to_string());
}

pub fn build_tokens() -> TokenStream2 {
    let collected_meta_tokens = MERGE_META
        .lock()
        .unwrap()
        .drain(..)
        .map(|tokens| {
            let tokens = TokenStream2::from_str(tokens.as_str()).unwrap();
            quote! {
                #tokens
            }
        })
        .collect::<Vec<TokenStream2>>();
    let collected_meta_tokens: TokenStream2 = TokenStream2::from(quote! {
        #(#collected_meta_tokens)*
    });
    collected_meta_tokens
}
