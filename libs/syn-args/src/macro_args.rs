use std::{collections::HashMap, fmt::Debug};

use quote::ToTokens;
// use def::Type;
use syn::Error;

pub mod def;
pub mod utils;

pub struct Formal {}

impl Formal {
    pub fn new() -> Self {
        Formal {}
    }

    pub fn parse(&self, input: &str) -> Result<Vec<Value>, Error> {
        let mut res: Vec<Value> = vec![];
        let input = utils::expr_fix(input);
        let expr = syn::parse_str::<syn::ExprCall>(&input).unwrap();
        // println!("{:#?}", expr.args);

        for arg in expr.args {
            match arg {
                syn::Expr::Lit(lit) => match lit.lit {
                    syn::Lit::Int(int) => {
                        let v = int.base10_parse::<i32>().unwrap();
                        res.push(Value::Int(def::Int(v)));
                    }
                    syn::Lit::Str(str) => {
                        let v = str.value();
                        res.push(Value::String(def::String(v)));
                    }
                    _ => {}
                },
                syn::Expr::Path(path) => {
                    res.push(Value::Ident(def::Ident(path.path.segments[0].ident.to_string())));
                }
                syn::Expr::Array(array) => {
                    let mut arr = vec![];
                    for item in array.elems {
                        match item {
                            syn::Expr::Lit(lit) => match lit.lit {
                                syn::Lit::Int(int) => {
                                    let v = int.base10_parse::<i32>().unwrap();
                                    arr.push(Value::Int(def::Int(v)));
                                }
                                syn::Lit::Str(str) => {
                                    let v = str.value();
                                    arr.push(Value::String(def::String(v)));
                                }
                                _ => {}
                            },
                            syn::Expr::Path(path) => {
                                arr.push(Value::Ident(def::Ident(path.path.segments[0].ident.to_string())));
                            }
                            _ => {}
                        }
                    }
                    res.push(Value::Array(def::Array(arr)));
                }
                syn::Expr::Struct(struct_expr) => {
                    let mut obj = HashMap::new();
                    for field in struct_expr.fields {
                        let key = field.member.to_token_stream().to_string();
                        match field.expr {
                            syn::Expr::Lit(lit) => match lit.lit {
                                syn::Lit::Int(int) => {
                                    let v = int.base10_parse::<i32>().unwrap();
                                    obj.insert(key, Value::Int(def::Int(v)));
                                }
                                syn::Lit::Str(str) => {
                                    let v = str.value();
                                    obj.insert(key, Value::String(def::String(v)));
                                }
                                _ => {}
                            },
                            syn::Expr::Path(path) => {
                                obj.insert(key, Value::Ident(def::Ident(path.path.segments[0].ident.to_string())));
                            }
                            syn::Expr::Array(array) => {
                                let mut arr = vec![];
                                for item in array.elems {
                                    match item {
                                        syn::Expr::Lit(lit) => match lit.lit {
                                            syn::Lit::Int(int) => {
                                                let v = int.base10_parse::<i32>().unwrap();
                                                arr.push(Value::Int(def::Int(v)));
                                            }
                                            syn::Lit::Str(str) => {
                                                let v = str.value();
                                                arr.push(Value::String(def::String(v)));
                                            }
                                            _ => {}
                                        },
                                        syn::Expr::Path(path) => {
                                            arr.push(Value::Ident(def::Ident(path.path.segments[0].ident.to_string())));
                                        }
                                        _ => {}
                                    }
                                }
                                obj.insert(key, Value::Array(def::Array(arr)));
                            }
                            _ => {}
                        }
                    }
                    res.push(Value::Object(def::Object(obj)));
                }
                _ => {}
            }
        }

        Ok(res)
    }
}

// #[derive(Debug)]
// pub struct DefArgument {
//     pub arg_type: Type,
//     pub desc: String,
//     pub required: bool,
//     pub default: Option<Value>,
// }

// impl DefArgument {
//     pub fn new(arg_type: Type, desc: &str, required: bool, default: Option<Value>) -> Self {
//         DefArgument { arg_type, desc: desc.to_string(), required, default }
//     }
// }

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

impl TryInto<def::Object<HashMap<String, Value>>> for &Value {
    type Error = Error;

    fn try_into(self) -> Result<def::Object<HashMap<String, Value>>, Self::Error> {
        match self {
            Value::Object(v) => {
                let mut res = HashMap::new();
                for (k, v) in v.0.iter() {
                    res.insert(k.clone(), v.clone());
                }
                Ok(def::Object(res))
            }
            _ => Err(Error::new(proc_macro2::Span::call_site(), "Expected Object")),
        }
    }
}
