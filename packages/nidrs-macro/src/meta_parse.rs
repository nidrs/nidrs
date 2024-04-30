use std::{collections::HashMap, str::FromStr, sync::Mutex};

use once_cell::sync::Lazy;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{parse_str, Expr};

use crate::MetaArgs;

static MERGE_META: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(vec![]));
static MATE_VALUE: Lazy<Mutex<HashMap<String, MetaValue>>> = Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Debug, Clone)]
pub enum MetaValue {
    String(String),
    Bool(bool),
    Int(i64),
    Float(f64),
    Array(Vec<MetaValue>),
    Object(HashMap<String, MetaValue>),
}

pub fn clear() {
    MERGE_META.lock().unwrap().clear();
    MATE_VALUE.lock().unwrap().clear();
}

fn exp_to_meta_value(exp: &Expr) -> MetaValue {
    match exp {
        Expr::Lit(lit) => {
            let lit = lit.lit.clone();
            if let syn::Lit::Str(s) = lit {
                MetaValue::String(s.value())
            } else if let syn::Lit::Bool(b) = lit {
                MetaValue::Bool(b.value)
            } else if let syn::Lit::Int(i) = lit {
                MetaValue::Int(i.base10_parse().unwrap())
            } else if let syn::Lit::Float(f) = lit {
                MetaValue::Float(f.base10_parse().unwrap())
            } else {
                MetaValue::String(lit.to_token_stream().to_string())
            }
        }
        Expr::Array(arr) => {
            let arr = arr.elems.iter().map(|elem| exp_to_meta_value(elem)).collect::<Vec<MetaValue>>();
            MetaValue::Array(arr)
        }
        Expr::Struct(s) => {
            let mut obj = HashMap::new();
            s.fields.iter().for_each(|field| {
                let k = field.member.to_token_stream().to_string();
                let v = exp_to_meta_value(&field.expr);
                obj.insert(k, v);
            });
            MetaValue::Object(obj)
        }
        _ => MetaValue::String(exp.to_token_stream().to_string()),
    }
}

pub fn collect(args: MetaArgs) {
    let meta_tokens = args
        .kv
        .iter()
        .map(|(key, exp)| {
            // println!("// meta {} {} {:?}", key, value, exp);

            let mv = exp_to_meta_value(exp.clone().as_ref());
            MATE_VALUE.lock().unwrap().insert(key.clone(), mv);

            // print META_VALUE
            // println!("META_VALUE: {:?}", MATE_VALUE.lock().unwrap());

            let v = match exp.as_ref() {
                Expr::Array(arr) => {
                    // arr to vec
                    let arr = arr.elems.iter().map(|elem| elem.clone().to_owned()).collect::<Vec<Expr>>();

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

pub fn get_meta_value(key: &str) -> Option<MetaValue> {
    MATE_VALUE.lock().unwrap().get(key).cloned()
}
