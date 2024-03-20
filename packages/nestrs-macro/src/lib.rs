extern crate proc_macro;

use proc_macro::{TokenStream, Span};
use syn::{parse_macro_input, ItemFn, ItemStruct};
use syn::punctuated::Punctuated;
use syn::parse::{Parse, ParseStream};
use quote::quote;

struct Args {
    vars: Vec<syn::Expr>
}

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        let vars = Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated(input)?;
        
        Ok(Args {
            vars: vars.into_iter().collect(),
        })
    }
}

impl Args {
    pub fn get_method(&self) -> syn::Result<syn::Expr> {
        match self.vars.get(0) {
            Some(var) => Ok(var.clone()),
            None => return Err(syn::Error::new(
                Span::call_site().into(),
                "No HTTP Method was provided"
            ))
        }
    }
    
    pub fn get_route(&self) -> syn::Result<syn::Expr> {
        match self.vars.get(1) {
            Some(var) => Ok(var.clone()),
            None => return Err(syn::Error::new(
                Span::call_site().into(),
                "No Route was provided"
            ))
        }
    }
}

#[proc_macro_attribute]
pub fn get(args: TokenStream, input: TokenStream) -> TokenStream {
    // let args = parse_macro_input!(args as Args);
    // let func = parse_macro_input!(input as ItemFn);
    
    // let vis = func.vis.clone();
    // let ident = func.sig.ident.clone();
    
    // let method = args.get_method().unwrap();
    // let route = args.get_route().unwrap();
    
    // let expanded = quote! {
    //     #[allow(non_camel_case_types)]
    //     #vis struct #ident;
        
    //     impl #ident {
    //         #vis fn route() -> axum::Router::<AppState> {
    //             #func
                
    //             axum::Router::new().route(#route, #method (#ident))
    //         }
    //     }
    // };
    
    // expanded.into()
    // 解析输入的结构体
    let input = parse_macro_input!(input as ItemFn);

    // 打印宏的参数
    // println!("Controller args: {:?}", attr_args);

    // 打印输入的结构体
    println!("Get: {:?}", input.sig.ident);

    // 返回原始的输入，因为我们并没有修改它
    TokenStream::from(quote! {
        #input
    })
}

#[proc_macro_attribute]
pub fn post(args: TokenStream, input: TokenStream) -> TokenStream {
    return  input;
}

#[proc_macro_attribute]
pub fn put(args: TokenStream, input: TokenStream) -> TokenStream {
    return  input;
}

#[proc_macro_attribute]
pub fn delete(args: TokenStream, input: TokenStream) -> TokenStream {
    return  input;
}



#[proc_macro_attribute]
pub fn controller(args: TokenStream, input: TokenStream) -> TokenStream {
    // 解析宏的参数
    // let attr_args = parse_macro_input!(args as );

    // 解析输入的结构体
    let input = parse_macro_input!(input as ItemStruct);

    // 打印宏的参数
    // println!("Controller args: {:?}", attr_args);

    // 打印输入的结构体
    println!("Controller input: {:?}", input.ident);

    // 返回原始的输入，因为我们并没有修改它
    TokenStream::from(quote! {
        #input
    })
}

#[proc_macro_attribute]
pub fn module(args: TokenStream, input: TokenStream) -> TokenStream {
    // 解析宏的参数
    // let attr_args = parse_macro_input!(args as );

    // 解析输入的结构体
    // let input2 = parse_macro_input!(input as ItemStruct);

    // 打印宏的参数
    // println!("Controller args: {:?}", attr_args);

    // 打印输入的结构体
    // println!("Controller input: {:?}", input2.ident);

    // 返回原始的输入，因为我们并没有修改它
    return input;
}