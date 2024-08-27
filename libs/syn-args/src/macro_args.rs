use std::{collections::HashMap, fmt::Debug};

use quote::ToTokens;
use syn::Error;

pub mod def;
pub mod utils;

pub struct Formal {}

impl Default for Formal {
    fn default() -> Self {
        Self::new()
    }
}

impl Formal {
    pub fn new() -> Self {
        Formal {}
    }

    pub fn parse(&self, input: &str) -> Result<Value, Error> {
        let mut res: Vec<Value> = vec![];
        let input = utils::expr_fix(input);
        let expr = syn::parse_str::<syn::ExprCall>(&input).unwrap();
        // println!("{:#?}", expr.args);

        for arg in expr.args {
            let value = recursive_parsing(&arg);
            res.push(value);
        }

        Ok(Value::Array(def::Array(res)))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    PathIdent(def::PathIdent),
    Int(def::Int),
    Float(def::Float),
    Bool(def::Bool),
    String(def::String),
    Option(def::Option<Box<Value>>),
    Object(def::Object<Value>),
    Array(def::Array<Value>),
}

pub fn recursive_parsing(input: &syn::Expr) -> Value {
    match input {
        syn::Expr::Lit(lit) => recursive_lit(&lit.lit),
        syn::Expr::Path(path) => Value::PathIdent(def::PathIdent(path.path.clone())),
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
        _ => Value::Null,
    }
}

pub fn recursive_lit(lit: &syn::Lit) -> Value {
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
