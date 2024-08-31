use std::collections::HashMap;

use quote::ToTokens;
use syn::Error;

use super::{def, Value};

pub fn otr<T>(opt: Option<T>) -> Result<T, Error> {
    match opt {
        Some(val) => Ok(val),
        None => Err(Error::new(proc_macro2::Span::call_site(), "Invalid args otr")),
    }
}

pub fn ewc<F, T, E>(callback: F) -> Result<T, E>
where
    F: FnOnce() -> Result<T, E>,
{
    // 调用闭包并返回结果
    callback()
}

pub(crate) fn recursive_parsing(input: &syn::Expr) -> Value {
    match input {
        syn::Expr::Lit(lit) => recursive_lit(&lit.lit),
        syn::Expr::Array(array) => {
            let mut arr = vec![];
            for item in array.elems.iter() {
                let item = recursive_parsing(item);
                arr.push(item);
            }
            Value::Array(def::Array(arr))
        }
        syn::Expr::Struct(struct_expr) => {
            let mut obj = HashMap::new();
            for field in struct_expr.fields.iter() {
                let key = field.member.to_token_stream().to_string();
                let value = recursive_parsing(&field.expr);
                obj.insert(key, value);
            }
            Value::Object(def::Object(obj))
        }
        _ => Value::Expr(def::Expr(input.clone())),
    }
}

pub(crate) fn recursive_lit(lit: &syn::Lit) -> Value {
    match lit {
        syn::Lit::Int(int) => {
            let v = int.base10_parse::<i32>().unwrap();
            Value::Int(def::Int(v))
        }
        syn::Lit::Str(str) => {
            let v = str.value();
            Value::String(def::String(v))
        }
        syn::Lit::Float(float) => {
            let v = float.base10_parse::<f32>().unwrap();
            Value::Float(def::Float(v))
        }
        syn::Lit::Bool(bool) => {
            let v = bool.value;
            Value::Bool(def::Bool(v))
        }
        _ => Value::Null,
    }
}
