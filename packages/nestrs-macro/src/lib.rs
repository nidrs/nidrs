#![allow(warnings, unused)]
extern crate proc_macro;

use std::{
    any::Any, borrow::BorrowMut, collections::HashMap, sync::{Arc, Mutex}
};

use once_cell::sync::Lazy;
use proc_macro::{Span, TokenStream};
use proc_macro2::Punct;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, spanned::Spanned, FnArg, ItemFn, ItemStruct, PatType, Type};

static mut CURRENT_CONTROLLER: Option<ControllerMeta> = None;
static mut ROUTES: Lazy<Arc<Mutex<HashMap<String, HashMap<String, RouteMeta>>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

// struct Args {
//     args: Punctuated<syn::Expr, Punct>
// }

#[derive(Debug, Clone)]
struct RouteMeta {
    method: String,
    path: String,
    name: String,
    handler: TokenStream,
}

#[derive(Debug, Clone)]
struct ControllerMeta {
    name: String,
    path: String,
}

#[proc_macro_attribute]
pub fn get(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as syn::Expr);
    let func = parse_macro_input!(input as ItemFn);

    let path = if let syn::Expr::Lit(lit) = args {
        if let syn::Lit::Str(str) = lit.lit {
            str.value().trim().to_string()
        } else {
            panic!("Invalid argument")
        }
    } else {
        // throw error
        panic!("Invalid argument");
    };
    let name = func.sig.ident.to_string();
    println!("Get: {:?}", path);

    let vis = func.vis.clone();
    let ident = func.sig.ident.clone();

    let func_args = func
        .sig
        .inputs
        .iter()
        .map(|arg| match arg {
            FnArg::Typed(PatType { pat, ty, .. }) => {
                let pat = pat.to_token_stream();
                let ty = ty.to_token_stream();
                quote! {
                    #pat: #ty
                }
            }
            _ => quote! {},
        })
        .reduce(|a, b| {
            if (a.to_string().is_empty()) {
                return b;
            }
            quote! {
                #a, #b
            }
        })
        .unwrap_or(quote! {});

    let route = RouteMeta {
        method: "get".to_string(),
        path: path,
        name: name.clone(),
        handler: TokenStream::from(quote! {
            |state| async move {
                t_controller.#ident(state).await
            }
        }),
    };
    
    unsafe {
        let routes_clone = ROUTES.clone();
        let controller = CURRENT_CONTROLLER.as_ref().unwrap();
        println!("routes_clone.lock");
        let mut routes = routes_clone.lock().unwrap();
        println!("routes_clone.locked");
        let mut controller_routes = routes.get_mut(&controller.name).unwrap();
        println!("routes_clone.insert");
        controller_routes.insert(name.clone(), route);
        println!("routes_clone.inserted");
        println!("Routes: {:?}", controller_routes.len());
        println!("routes_clone.read");
    };

    // println!("Get: {:?}, Params: {:?}", ident, func_args.to_string());
    // println!("Get: {:?}, Params {:?}", ident, func.sig.inputs.first().unwrap());

    // 返回原始的输入，因为我们并没有修改它
    TokenStream::from(quote! {
        #func
    })
}

#[proc_macro_attribute]
pub fn post(args: TokenStream, input: TokenStream) -> TokenStream {
    return input;
}

#[proc_macro_attribute]
pub fn put(args: TokenStream, input: TokenStream) -> TokenStream {
    return input;
}

#[proc_macro_attribute]
pub fn delete(args: TokenStream, input: TokenStream) -> TokenStream {
    return input;
}

#[proc_macro_attribute]
pub fn controller(args: TokenStream, input: TokenStream) -> TokenStream {
    // 解析宏的参数
    let args = parse_macro_input!(args as syn::Expr);
    let path = if let syn::Expr::Lit(lit) = args {
        if let syn::Lit::Str(str) = lit.lit {
            str.value().trim().to_string()
        } else {
            panic!("Invalid argument")
        }
    } else {
        // throw error
        panic!("Invalid argument");
    };

    // 解析输入的结构体
    let input = parse_macro_input!(input as ItemStruct);

    let ident = input.ident.clone();

    unsafe {
        CURRENT_CONTROLLER = Some(ControllerMeta {
            name: ident.to_string(),
            path: path,
        });
        ROUTES
            .clone()
            .lock()
            .unwrap()
            .insert(ident.to_string(), HashMap::new());
    }

    // 打印宏的参数
    // println!("Controller args: {:?}", attr_args);

    // 打印输入的结构体
    println!(
        "Controller: {:?}", ident.to_string()
    );

    // 返回原始的输入，因为我们并没有修改它
    TokenStream::from(quote! {
        #input
    })
}

#[proc_macro_attribute]
pub fn module(args: TokenStream, input: TokenStream) -> TokenStream {
    // 解析宏的参数
    let func = parse_macro_input!(input as ItemStruct);

    println!("Module: {:?}, Router: {:?}", func.ident, unsafe {
        ROUTES.lock().unwrap().iter().map(|(k, v)| {
            (k, v.len())
        }).collect::<Vec<(&String, usize)>>()
    });

    unsafe {
        let mut routes = ROUTES.lock().unwrap();
        // routes.clear();
        CURRENT_CONTROLLER = None;
    };

    // 返回原始的输入，因为我们并没有修改它
    return TokenStream::from(quote! {
        #func
    });
}
#[proc_macro]
pub fn get_route_meta(input: TokenStream) -> TokenStream {
    return input;
}
