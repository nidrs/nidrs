use std::collections::HashMap;

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream}, punctuated::Punctuated, spanned::Spanned, Expr, ExprCall, FieldValue, Ident, ItemFn, ItemStruct, LitBool, Member, Token
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


#[derive(Debug, Clone)]
pub struct ModuleArgs {
    pub imports: Vec<TokenStream2>,
    pub controllers: Vec<TokenStream2>,
    pub services: Vec<TokenStream2>,
    pub exports: Vec<TokenStream2>,
    pub interceptors: Vec<TokenStream2>,
}

impl Parse for ModuleArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let _ = syn::braced!(content in input);

        let mut fields: Punctuated<FieldValue, Token![,]> = Punctuated::new();

        while !content.is_empty() {
            fields.push(content.parse()?);
            if content.is_empty() {
                break;
            }
            let punct: Token![,] = content.parse()?;
            fields.push_punct(punct);
        }

        let mut imports = Vec::new();
        let mut controllers = Vec::new();
        let mut services = Vec::new();
        let mut exports = Vec::new();
        let mut interceptors = Vec::new();

        fields.iter().for_each(|field| {
            if let Member::Named(field_ident) = &field.member {
                match field_ident.to_string().as_str() {
                    "imports" => {
                        if let syn::Expr::Array(array) = &field.expr {
                            array.elems.iter().for_each(|elem| {
                                imports.push(elem.to_token_stream());
                            });
                        }
                    }
                    "controllers" => {
                        if let syn::Expr::Array(array) = &field.expr {
                            array.elems.iter().for_each(|elem| {
                                controllers.push(elem.to_token_stream());
                            });
                        }
                    }
                    "services" => {
                        if let syn::Expr::Array(array) = &field.expr {
                            array.elems.iter().for_each(|elem| {
                                services.push(elem.to_token_stream());
                            });
                        }
                    }
                    "exports" => {
                        if let syn::Expr::Array(array) = &field.expr {
                            array.elems.iter().for_each(|elem| {
                                exports.push(elem.to_token_stream());
                            });
                        }
                    }
                    "interceptors" => {
                        if let syn::Expr::Array(array) = &field.expr {
                            array.elems.iter().for_each(|elem| {
                                interceptors.push(elem.to_token_stream());
                            });
                        }
                    }
                    _ => (),
                }
            }
            // println!("{}", json::to_string_pretty(field));
            // println!("{}", json::to_string_pretty(field));
            // let field_ident = field.ident.clone().unwrap().to_string();
            // match field {

            // }
        });

        // parse_args_map.iter().for_each(|(k, v)| match k.as_str() {
        //     "imports" => imports = v.clone(),
        //     "controllers" => controllers = v.clone(),
        //     "services" => services = v.clone(),
        //     "exports" => exports = v.clone(),
        //     "interceptors" => interceptors = v.clone(),
        //     _ => {}
        // });

        // nidrs_macro::log!("{:?}", parse_args_map);

        Ok(ModuleArgs { imports, controllers, services, exports, interceptors })
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
                imports: [ControllerService, InterceptorService::for_root()],
                services: [ServiceA, ServiceB],
            }
        };
        let taste: ModuleArgs = parse2(t).unwrap();

        assert!(taste.imports.len() == 2);
        assert!(taste.services.len() == 2);
    }
}


