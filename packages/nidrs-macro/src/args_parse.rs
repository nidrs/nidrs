use std::collections::HashMap;

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    token::{Paren, Token},
    Expr, ExprCall, FieldValue, Ident, ItemFn, ItemStruct, LitBool, Member, Token,
};
use syn_serde::json;

#[derive(Clone)]
pub struct ExprList {
    pub items: Punctuated<Expr, syn::Token![,]>,
}

impl Parse for ExprList {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let items = Punctuated::parse_terminated(input)?;
        Ok(ExprList { items })
    }
}

#[derive(Clone)]
pub struct MetaArgs {
    pub kv: HashMap<String, Box<Expr>>,
}

impl Parse for MetaArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let items: Punctuated<Expr, syn::Token![,]> = Punctuated::parse_terminated(input)?;
        let mut kv = HashMap::new();
        items.iter().for_each(|item| {
            if let syn::Expr::Assign(assign) = item {
                if let syn::Expr::Path(path) = *assign.left.clone() {
                    let k = path.path.segments.first().unwrap().ident.to_string();
                    let v = assign.right.clone();
                    kv.insert(k, v);
                }
            } else if let syn::Expr::Path(path) = item {
                let mut p = path.path.segments.iter().map(|s| s.ident.to_string()).collect::<Vec<String>>().join("::");
                if p.contains("Global::") {
                    if p.contains("Global::Enabled") {
                        p = "Global::Enabled".to_string();
                    } else {
                        p = "Global::Disabled".to_string();
                    }
                } else if p.contains("DefaultPrefix::") {
                    if p.contains("DefaultPrefix::Disabled") {
                        p = "DefaultPrefix::Disabled".to_string();
                    } else {
                        p = "DefaultPrefix::Enabled".to_string();
                    }
                }
                let key = format!("{}:{}", "METADATA", p).replace(" ", "");
                kv.insert(key, Box::new(path.clone().into()));
            } else if let syn::Expr::Call(call) = item {
                // println!("p {:?}", p);
                let mut p = call.func.to_token_stream().to_string();
                let key = format!("{}:{}", "METADATA", p).replace(" ", "");
                kv.insert(key, Box::new(call.clone().into()));
            } else {
                println!("metaArgs {:?}", item);
                panic!("Invalid argument");
            }
        });
        Ok(MetaArgs { kv })
    }
}

#[derive(Clone)]
pub enum TokenType {
    Fn(ItemFn),
    Struct(ItemStruct),
}
#[derive(Clone)]
pub struct InterceptorArgs {
    pub ident: Ident,
    pub typ: TokenType,
}

impl Parse for InterceptorArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // 使用 peek 方法来检查输入的下一个 Token 类型
        let struct_parse = input.parse::<syn::ItemStruct>();
        let fn_parse = input.parse::<syn::ItemFn>();
        if let Ok(item) = struct_parse {
            Ok(InterceptorArgs { ident: item.clone().ident, typ: TokenType::Struct(item) })
        } else if let Ok(item) = fn_parse {
            Ok(InterceptorArgs { ident: item.sig.ident.clone(), typ: TokenType::Fn(item) })
        } else {
            Err(syn::Error::new(input.span(), "Invalid interceptor"))
        }
    }
}

#[derive(Clone)]
pub struct DefaultUsesOptions {
    pub args: Vec<TokenStream2>,
}

impl Parse for DefaultUsesOptions {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // eg: default_uses(xxx)
        let ident: Ident = input.parse()?;

        let content;
        let _ = syn::parenthesized!(content in input);

        let mut fields: Punctuated<Expr, Token![,]> = Punctuated::new();

        while !content.is_empty() {
            fields.push(content.parse()?);
            if content.is_empty() {
                break;
            }
            let punct: Token![,] = content.parse()?;
            fields.push_punct(punct);
        }

        Ok(DefaultUsesOptions { args: fields.iter().map(|field| field.to_token_stream()).collect() })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use proc_macro::TokenStream;
    use proc_macro2::TokenStream as TokenStream2;
    use quote::ToTokens;
    use syn::{parse, parse2, parse_macro_input, Block, ExprStruct, ItemFn, ItemStruct, Stmt};
    use syn_serde::json;

    #[test]
    fn test_args_parse() {
        let t = quote::quote! {
            {
                imports: [Controller, Interceptor::for_root()],
                services: [ServiceA, ServiceB],
            }
        };
        let taste: ModuleArgs = parse2(t).unwrap();

        assert!(taste.imports.len() == 2);
        assert!(taste.services.len() == 2);
    }
}
