use std::collections::HashMap;

use def::Type;
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
                        res.push(Value::Int(int.base10_parse::<i32>().unwrap()));
                    }
                    syn::Lit::Str(str) => {
                        res.push(Value::String(str.value()));
                    }
                    _ => {}
                },
                syn::Expr::Array(array) => {
                    let mut arr = vec![];
                    for item in array.elems {
                        match item {
                            syn::Expr::Lit(lit) => match lit.lit {
                                syn::Lit::Int(int) => {
                                    arr.push(Value::Int(int.base10_parse::<i32>().unwrap()));
                                }
                                syn::Lit::Str(str) => {
                                    arr.push(Value::String(str.value()));
                                }
                                _ => {}
                            },
                            syn::Expr::Path(path) => {
                                arr.push(Value::Ident(path.path.segments[0].ident.to_string()));
                            }
                            _ => {}
                        }
                    }
                    res.push(Value::Array(arr));
                }
                syn::Expr::Path(path) => {
                    res.push(Value::Ident(path.path.segments[0].ident.to_string()));
                }
                _ => {}
            }
        }

        Ok(res)
    }
}

#[derive(Debug)]
pub struct DefArgument {
    pub arg_type: Type,
    pub desc: String,
    pub required: bool,
    pub default: Option<Value>,
}

impl DefArgument {
    pub fn new(arg_type: Type, desc: &str, required: bool, default: Option<Value>) -> Self {
        DefArgument { arg_type, desc: desc.to_string(), required, default }
    }
}

#[derive(Debug)]
pub enum Value {
    Null,
    Ident(String),
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),
    Object(HashMap<String, Value>),
    Array(Vec<Value>),
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
                for item in arr {
                    let item: Item = item.try_into()?;
                    res.push(item);
                }
                Ok(def::Array(res))
            }
            _ => Err(Error::new(proc_macro2::Span::call_site(), "Expected Array")),
        }
    }
}
