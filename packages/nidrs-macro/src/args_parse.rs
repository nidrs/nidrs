use std::collections::HashMap;

use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{parse::{Parse, ParseStream}, punctuated::Punctuated, Expr, Ident, ItemFn, ItemStruct, Token};


#[derive(Debug, Clone)]
pub struct ExprList {
  pub items: Punctuated<Expr, syn::Token![,]>,
}

impl Parse for ExprList {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
      let items = Punctuated::parse_terminated(input)?;
      Ok(ExprList { items })
  }
}

#[derive(Debug, Clone)]
pub struct MetaArgs {
    pub kv: HashMap<String, String>,
}

impl Parse for MetaArgs {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
      let items: Punctuated<Expr, syn::Token![,]> = Punctuated::parse_terminated(input)?;
        let mut kv = HashMap::new();
        items.iter().for_each(|item| {
            if let syn::Expr::Assign(assign) = item {
                if let syn::Expr::Path(path) = *assign.left.clone() {
                    if let syn::Expr::Lit(lit) = *assign.right.clone() {
                        kv.insert(path.path.segments.first().unwrap().ident.to_string(), lit.to_token_stream().to_string());
                    }
                }
            }
        });
        Ok(MetaArgs {
            kv,
        })
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
        let args:syn::Block = input.parse()?;
        // let args = parse_macro_input!(input.parse::<Expr>()) as Expr;

        let mut imports = Vec::new();
        let mut controllers = Vec::new();
        let mut services = Vec::new();
        let mut exports = Vec::new();
        let mut interceptors = Vec::new();

        let parse_args_map = args.stmts.iter().map(|stmt| {
            if let syn::Stmt::Expr(exp, _) = stmt {
                if let syn::Expr::Assign(assign) = exp {
                    if let syn::Expr::Path(path) = *assign.left.clone() {
                        return (path.path.segments.first().unwrap().ident.to_string(), if let syn::Expr::Array(array) = *assign.right.clone() {
                            array.elems.iter().map(|elem| {
                                if let syn::Expr::Path(path) = elem {
                                    return path.path.segments.first().unwrap().ident.to_token_stream()
                                }
                                if let syn::Expr::Call(lit) = elem {
                                    return lit.to_token_stream()
                                }
                                return TokenStream2::new();
                            }).collect::<Vec<TokenStream2>>()
                        } else {
                            vec![]
                        });
                    }
                }
    
            }
            panic!("Invalid argument");
        }).collect::<HashMap<String, Vec<TokenStream2>>>();

        parse_args_map.iter().for_each(|(k, v)| {
            match k.as_str() {
                "imports" => imports = v.clone(),
                "controllers" => controllers = v.clone(),
                "services" => services = v.clone(),
                "exports" => exports = v.clone(),
                "interceptors" => interceptors = v.clone(),
                _ => {}
            }
        });
        
        // nidrs_macro::log!("{:?}", parse_args_map);
    
        Ok(ModuleArgs {
            imports,
            controllers,
            services,
            exports,
            interceptors,
        })
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
          Ok(InterceptorArgs {
              ident: item.clone().ident,
              typ: TokenType::Struct(item),
          })
      } else if let Ok(item) = fn_parse {
          Ok(InterceptorArgs {
              ident: item.sig.ident.clone(),
              typ: TokenType::Fn(item),
          })
      } else {
          Err(syn::Error::new(input.span(), "Invalid interceptor"))
      }
  }
}
