use std::ops::Deref;

use proc_macro2::Group;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    token::{Brace, Paren},
    Expr, Ident, Lit, Path, Token,
};

use crate::{def, recursive_lit, recursive_parsing, Value};

#[derive(Debug)]
pub struct SynArgs {
    pub value: Value,
}

impl Parse for SynArgs {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let mut res: Vec<Value> = vec![];

        let option_content;
        let content = if input.peek(Paren) {
            let content_t;
            let _ = syn::parenthesized!(content_t in input);
            option_content = Some(content_t);
            option_content.as_ref().unwrap()
        } else {
            input
        };

        while !content.is_empty() {
            if content.peek(Lit) {
                let lit = content.parse::<Lit>()?;
                res.push(recursive_lit(&lit));
            } else if content.peek(Ident) {
                let ident = content.parse::<Path>()?;
                res.push(Value::Ident(def::Ident(ident.to_token_stream().to_string())));
            } else if content.peek(Brace) {
                let group: Group = content.parse()?;

                let stream = group.stream();

                let p: ObjectArgs = syn::parse2(stream).unwrap();

                res.push(Value::Object(def::Object(p.kv)));
            } else if let Ok(v) = content.parse::<syn::Expr>() {
                res.push(recursive_parsing(&v));
            }

            if content.is_empty() {
                break;
            }
            let _ = content.parse::<Token![,]>()?;
        }

        Ok(SynArgs { value: Value::Array(def::Array(res)) })
    }
}

impl Deref for SynArgs {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

pub struct ObjectArgs {
    pub kv: std::collections::HashMap<String, Value>,
}

impl Parse for ObjectArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content = input;

        let mut kv = std::collections::HashMap::new();

        while !content.is_empty() {
            let key = content.parse::<Ident>()?;
            let _: Token![:] = content.parse()?;
            let value = content.parse::<Expr>()?;
            kv.insert(key.to_string(), recursive_parsing(&value));
            if content.is_empty() {
                break;
            }
            let _: Token![,] = content.parse()?;
        }

        Ok(ObjectArgs { kv })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_syn_args() {
        let args = syn::parse_str::<SynArgs>("(1, 20, { a: 12 })").unwrap();
        assert_eq!(format!("{:?}", args.value), "Array(Array([Int(Int(1)), Int(Int(20)), Object(Object({\"a\": Int(Int(12))}))]))");
    }

    #[test]
    fn test_syn_args2() {
        let args = syn::parse_str::<SynArgs>("(Test, MY::TEST)").unwrap();
        assert_eq!(format!("{:?}", args.value), "Array(Array([Ident(Ident(\"Test\")), Ident(Ident(\"MY :: TEST\"))]))");
    }

    #[test]
    fn test_syn_args3() {
        let args = syn::parse_str::<SynArgs>("([Test, MY::TEST])").unwrap();
        assert_eq!(format!("{:?}", args.value), "Array(Array([Array(Array([Ident(Ident(\"Test\")), Ident(Ident(\"MY\"))]))]))");
    }

    #[test]
    fn test_syn_args3_no_parenthesized() {
        let args = syn::parse_str::<SynArgs>("[Test, MY::TEST]").unwrap();
        assert_eq!(format!("{:?}", args.value), "Array(Array([Array(Array([Ident(Ident(\"Test\")), Ident(Ident(\"MY\"))]))]))");
    }
}
