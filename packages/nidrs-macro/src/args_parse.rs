use std::collections::HashMap;

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    token::{Paren, Token},
    Expr, ExprCall, FieldValue, Ident, ItemFn, ItemStruct, LitBool, Member, Token,
};
use syn_serde::json;

#[derive(Clone)]
pub enum TokenType {
    Fn(ItemFn),
    Struct(ItemStruct),
}
#[derive(Clone)]
pub struct UFnStruct {
    pub ident: Ident,
    pub typ: TokenType,
}

impl Parse for UFnStruct {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // 使用 peek 方法来检查输入的下一个 Token 类型
        let struct_parse = input.parse::<syn::ItemStruct>();
        let fn_parse = input.parse::<syn::ItemFn>();
        if let Ok(item) = struct_parse {
            Ok(UFnStruct { ident: item.ident.clone(), typ: TokenType::Struct(item) })
        } else if let Ok(item) = fn_parse {
            Ok(UFnStruct { ident: item.sig.ident.clone(), typ: TokenType::Fn(item) })
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
