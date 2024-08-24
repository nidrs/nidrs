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

pub fn recursive_parsing(input: &syn::Expr) -> Value {
    match input {
        syn::Expr::Lit(lit) => match &lit.lit {
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
        },
        syn::Expr::Path(path) => Value::Ident(def::Ident(path.path.segments[0].ident.to_string())),
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

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Ident(def::Ident),
    Int(def::Int),
    Float(def::Float),
    Bool(def::Bool),
    String(def::String),
    Object(def::Object<HashMap<String, Value>>),
    Array(def::Array<Value>),
}

impl<Item> TryInto<def::Array<Item>> for &Value
where
    Item: TryFrom<Self, Error = Error>,
{
    type Error = Error;

    fn try_into(self) -> Result<def::Array<Item>, Self::Error> {
        match self {
            Value::Array(arr) => {
                let mut res = vec![];
                for item in arr.0.iter() {
                    let item: Item = item.try_into()?;
                    res.push(item);
                }
                Ok(def::Array(res))
            }
            _ => Err(Error::new(proc_macro2::Span::call_site(), "Expected Array")),
        }
    }
}

impl TryFrom<&Value> for def::Object<HashMap<String, Value>> {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Object(obj) => {
                let mut res = HashMap::new();
                for (k, v) in obj.0.iter() {
                    res.insert(k.clone(), v.clone());
                }
                Ok(def::Object(res))
            }
            _ => Err(Error::new(proc_macro2::Span::call_site(), "Expected ModuleSubObj")),
        }
    }
}

// impl<T> TryFrom<&Value> for def::Object<T> {
//     type Error = Error;

//     fn try_from(value: &Value) -> Result<Self, Self::Error> {
//         match value {
//             Value::Object(obj) => {
//                 let imports = obj.0.get("imports").ok_or(Error::new(proc_macro2::Span::call_site(), "Expected imports"))?.try_into()?;
//                 Ok(def::Object(ModuleSubObj { imports }))
//             }
//             _ => Err(Error::new(proc_macro2::Span::call_site(), "Expected ModuleSubObj")),
//         }
//     }
// }
