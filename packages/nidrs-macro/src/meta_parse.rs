use std::{collections::HashMap, str::FromStr, sync::Mutex};

use nidrs_extern::datasets::{DisableDefaultPrefix, Global};
use once_cell::sync::Lazy;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{parse_str, Expr, ExprCall};

use crate::MetaArgs;

static MERGE_META: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(vec![]));
static MATE_VALUE: Lazy<Mutex<HashMap<String, MetaValue>>> = Lazy::new(|| Mutex::new(HashMap::new()));
static MATE_STACK: Lazy<Mutex<Vec<HashMap<String, MetaValue>>>> = Lazy::new(|| Mutex::new(Vec::new()));

#[derive(Debug, Clone)]
pub enum Metadata {
    DisableDefaultPrefix(DisableDefaultPrefix),
    Global(Global),
}

#[derive(Debug, Clone)]
pub enum MetaValue {
    None,
    String(String),
    Bool(bool),
    Int(i64),
    Float(f64),
    Array(Vec<MetaValue>),
    Object(HashMap<String, MetaValue>),
    Metadata(String, Box<MetaValue>),
}

impl Into<TokenStream2> for MetaValue {
    fn into(self) -> TokenStream2 {
        match self {
            MetaValue::String(s) => {
                let s = s.as_str();
                quote! {
                    #s.to_string()
                }
            }
            MetaValue::Bool(b) => {
                quote! {
                    #b
                }
            }
            MetaValue::Int(i) => {
                quote! {
                    #i
                }
            }
            MetaValue::Float(f) => {
                quote! {
                    #f
                }
            }
            _ => {
                quote! {}
            }
        }
    }
}

pub fn stash() {
    MATE_STACK.lock().unwrap().push(MATE_VALUE.lock().unwrap().clone());
    clear_meta();
}

pub fn clear_meta() {
    MERGE_META.lock().unwrap().clear();
    MATE_VALUE.lock().unwrap().clear();
}

pub fn clear() {
    clear_meta();
    MATE_STACK.lock().unwrap().clear();
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
        Expr::Call(call) => {
            if let Some(datasets) = tokens_to_metadata(call) {
                match datasets {
                    Metadata::DisableDefaultPrefix(disable_default_prefix) => {
                        return MetaValue::Metadata("DisableDefaultPrefix".to_string(), Box::new(MetaValue::Bool(disable_default_prefix.0)));
                    }
                    Metadata::Global(global) => {
                        return MetaValue::Metadata("Global".to_string(), Box::new(MetaValue::Bool(global.0)));
                    }
                }
            }
            MetaValue::String(exp.to_token_stream().to_string())
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
            // println!(" mv {} {:?}", key, mv);
            if let MetaValue::Metadata(k, v) = mv {
                MATE_VALUE.lock().unwrap().insert(k, *v);
            } else {
                MATE_VALUE.lock().unwrap().insert(key.clone(), mv);
            }

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
            if key.starts_with("METADATA:") {
                quote! {
                    meta.set_data(#v);
                }
            } else {
                quote! {
                    meta.set(#key.to_string(), #v);
                }
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
    let v = MATE_VALUE.lock().unwrap().get(key).cloned();
    if let Some(v) = v {
        return Some(v);
    }
    let mut stack = MATE_STACK.lock().unwrap();
    // println!("stack.len() {:?}", stack.len());
    if stack.len() > 0 {
        for i in (0..stack.len()).rev() {
            if stack[i].contains_key(key) {
                return stack[i].get(key).cloned();
            }
        }
    }
    None
}

pub fn has_meta_value(key: &str) -> bool {
    MATE_VALUE.lock().unwrap().contains_key(key)
}

pub fn add_meta_value(key: &str, value: MetaValue) {
    MATE_VALUE.lock().unwrap().insert(key.to_string(), value);
}

fn tokens_to_metadata(expr_call: &ExprCall) -> Option<Metadata> {
    let p = expr_call.func.to_token_stream().to_string().replace(" ", "");

    if p.contains("DisableDefaultPrefix") {
        let args = expr_call.args.clone();
        let args = args
            .iter()
            .map(|arg| {
                let mv: MetaValue = exp_to_meta_value(arg);
                mv
            })
            .collect::<Vec<MetaValue>>();
        if let Some(MetaValue::Bool(b)) = args.first() {
            return Some(Metadata::DisableDefaultPrefix(nidrs_extern::datasets::DisableDefaultPrefix(*b)));
        }
        return Some(Metadata::DisableDefaultPrefix(nidrs_extern::datasets::DisableDefaultPrefix(true)));
    }

    if p.contains("Global") {
        let args = expr_call.args.clone();
        let args = args
            .iter()
            .map(|arg| {
                let mv: MetaValue = exp_to_meta_value(arg);
                mv
            })
            .collect::<Vec<MetaValue>>();
        if let Some(MetaValue::Bool(b)) = args.first() {
            return Some(Metadata::Global(nidrs_extern::datasets::Global(*b)));
        }
        return Some(Metadata::Global(nidrs_extern::datasets::Global(true)));
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trans_metadata() {
        let call_expr = "nidrs::datasets::DisableDefaultPrefix(true)";
        let expr_call: ExprCall = syn::parse_str(call_expr).unwrap();

        println!("expr_call: {:?}", tokens_to_metadata(&expr_call));
    }
}
