use proc_macro2::Group;
use syn::{
    parse::{Parse, ParseStream},
    token::{Brace, Bracket, Paren},
    Expr, Ident, Lit, Token,
};

use crate::{def, recursive_lit, recursive_parsing, Value};

#[derive(Debug)]
pub struct SynArgs {
    pub value: Value,
}

impl SynArgs {
    pub fn arguments<T: TryFrom<Value, Error = syn::Error>>(self) -> Result<T, syn::Error> {
        T::try_from(self.value)
    }
}

impl Parse for SynArgs {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let mut res: Vec<Value> = vec![];

        if input.peek(Ident) && input.peek2(Paren) {
            input.parse::<Ident>()?;
        }

        if input.peek(Paren) {
            let group = input.parse::<Group>()?;
            let stream = group.stream();
            return syn::parse2(stream);
        }

        let content = input;

        while !content.is_empty() {
            if content.peek(Lit) {
                let lit = content.parse::<Lit>()?;
                res.push(recursive_lit(&lit));
            } else if content.peek(Brace) {
                let group: ObjectArgs = content.parse()?;
                res.push(Value::Object(def::Object(group.value)));
            } else if content.peek(Bracket) {
                let group: ArrayArgs = content.parse()?;
                res.push(Value::Array(def::Array(group.value)));
            } else if let Ok(v) = content.parse::<syn::Expr>() {
                res.push(recursive_parsing(&v));
            } else {
                println!("Failed to parse: {:?}", content);
            }

            if content.is_empty() {
                break;
            }
            let _ = content.parse::<Token![,]>()?;
        }

        Ok(SynArgs { value: Value::Array(def::Array(res)) })
    }
}

pub struct ObjectArgs {
    pub value: std::collections::HashMap<String, Value>,
}

impl Parse for ObjectArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let _ = syn::braced!(content in input);

        let mut value = std::collections::HashMap::new();

        while !content.is_empty() {
            let key: Ident = content.parse()?;
            let _: Token![:] = content.parse()?;
            if content.peek(Brace) {
                let group: ObjectArgs = content.parse()?;
                value.insert(key.to_string(), Value::Object(def::Object(group.value)));
            } else {
                let expr = content.parse::<Expr>()?;
                value.insert(key.to_string(), recursive_parsing(&expr));
            }
            if content.is_empty() {
                break;
            }
            let _: Token![,] = content.parse()?;
        }

        Ok(ObjectArgs { value })
    }
}

pub struct ArrayArgs {
    pub value: Vec<Value>,
}

impl Parse for ArrayArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let _ = syn::bracketed!(content in input);

        let mut value = vec![];

        while !content.is_empty() {
            let p = content.parse::<SynArgs>()?;

            if let Value::Array(def::Array(v)) = p.value {
                value.extend(v);
            }

            if content.is_empty() {
                break;
            }
            let _: Token![,] = content.parse()?;
        }

        Ok(ArrayArgs { value })
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
        assert_eq!(format!("{:?}", args.value), "Array(Array([Expr(Expr(\"Test\")), Expr(Expr(\"MY :: TEST\"))]))");
    }

    #[test]
    fn test_syn_args3() {
        let args = syn::parse_str::<SynArgs>("([Test, MY::TEST])").unwrap();
        assert_eq!(format!("{:?}", args.value), "Array(Array([Array(Array([Expr(Expr(\"Test\")), Expr(Expr(\"MY :: TEST\"))]))]))");
    }

    #[test]
    fn test_syn_args3_no_parenthesized() {
        let args = syn::parse_str::<SynArgs>("[Test, MY::TEST]").unwrap();
        assert_eq!(format!("{:?}", args.value), "Array(Array([Array(Array([Expr(Expr(\"Test\")), Expr(Expr(\"MY :: TEST\"))]))]))");
    }
}
