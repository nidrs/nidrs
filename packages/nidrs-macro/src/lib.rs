#![allow(warnings, unused)]
#![feature(proc_macro_span)]
extern crate proc_macro;

use std::{
    any::Any,
    borrow::BorrowMut,
    cell::RefCell,
    collections::HashMap,
    ops::Add,
    path,
    str::FromStr,
    sync::{Arc, Mutex},
};

use cmeta::CMetaValue;
use nidrs_extern::datasets::ServiceType;
use once_cell::sync::Lazy;
use proc_macro::{Ident, Span, TokenStream};
use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::{Punct, TokenTree};
use quote::{quote, ToTokens};
use syn::{
    meta,
    parse::{Parse, ParseStream},
    parse_str, Expr, ExprArray, ExprCall, PatPath, Stmt, Token,
};
use syn::{parse, punctuated::Punctuated};
use syn::{parse_macro_input, spanned::Spanned, FnArg, ItemFn, ItemStruct, PatType, Type};

mod args_parse;
use args_parse::*;
mod global;

use crate::meta_parse::MetaValue;

mod app_parse;
mod cmeta;
mod g_current_module;
mod import_path;
mod meta_parse;
mod utils;

// static CURRENT_MODULE3: Mutex<Option<&mut CurrentModule>> = Mutex::new(None);

static CURRENT_CONTROLLER: Mutex<Option<ControllerMeta>> = Mutex::new(None);
static ROUTES: Lazy<Mutex<HashMap<String, HashMap<String, RouteMeta>>>> = Lazy::new(|| Mutex::new(HashMap::new())); // HashMap<ControllerName, HashMap<RouteName, RouteMeta>>
static CURRENT_SERVICE: Mutex<Option<ServiceMeta>> = Mutex::new(None);
static EVENTS: Lazy<Mutex<HashMap<String, Vec<(String, String)>>>> = Lazy::new(|| Mutex::new(HashMap::new())); // HashMap<EventName, Vec<(ServiceName,FName)>>
static INTERS: Lazy<Mutex<HashMap<String, Vec<String>>>> = Lazy::new(|| Mutex::new(HashMap::new())); // HashMap<ServiceName, Vec<InterName>>

static DEFAULT_INTERS: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(vec![]));

#[derive(Debug, Clone)]
struct RouteMeta {
    method: String,
    path: String,
    name: String,
    func_args: Vec<String>,
    is_body: bool,
    is_meta: bool,
}

#[derive(Debug, Clone)]
struct ControllerMeta {
    name: String,
    path: String,
}

#[derive(Debug, Clone)]
struct ServiceMeta {
    name: String,
}

#[proc_macro_attribute]
pub fn get(args: TokenStream, input: TokenStream) -> TokenStream {
    return route("get", args, input);
}

#[proc_macro_attribute]
pub fn post(args: TokenStream, input: TokenStream) -> TokenStream {
    return route("post", args, input);
}

#[proc_macro_attribute]
pub fn put(args: TokenStream, input: TokenStream) -> TokenStream {
    return route("put", args, input);
}

#[proc_macro_attribute]
pub fn delete(args: TokenStream, input: TokenStream) -> TokenStream {
    return route("delete", args, input);
}

#[proc_macro_attribute]
pub fn any(args: TokenStream, input: TokenStream) -> TokenStream {
    return route("any", args, input);
}

#[proc_macro_attribute]
pub fn head(args: TokenStream, input: TokenStream) -> TokenStream {
    return route("head", args, input);
}

#[proc_macro_attribute]
pub fn on(args: TokenStream, input: TokenStream) -> TokenStream {
    return route("on", args, input);
}

#[proc_macro_attribute]
pub fn options(args: TokenStream, input: TokenStream) -> TokenStream {
    return route("options", args, input);
}

#[proc_macro_attribute]
pub fn patch(args: TokenStream, input: TokenStream) -> TokenStream {
    return route("patch", args, input);
}

#[proc_macro_attribute]
pub fn trace(args: TokenStream, input: TokenStream) -> TokenStream {
    return route("trace", args, input);
}

#[proc_macro_attribute]
pub fn controller(args: TokenStream, input: TokenStream) -> TokenStream {
    g_current_module::begin_mod();

    // 解析宏的参数
    let path = if args.is_empty() {
        "".to_string()
    } else {
        let args = parse_macro_input!(args as syn::Expr);
        let path = if let syn::Expr::Lit(lit) = args {
            if let syn::Lit::Str(str) = lit.lit {
                str.value().trim().to_string()
            } else {
                panic!("Invalid argument")
            }
        } else {
            "".to_string()
        };
        path
    };

    let func = parse_macro_input!(input as ItemStruct);

    let ident = func.ident.clone();
    let ident_name = ident.to_string();

    import_path::push_path(&func.ident.to_string());

    println!("// controller {} {:?}", ident.to_string(), func.attrs);
    CURRENT_CONTROLLER.lock().unwrap().replace(ControllerMeta { name: ident.to_string(), path: path.clone() });
    ROUTES.lock().unwrap().insert(ident.to_string(), HashMap::new());

    TokenStream::from(quote! {
        #[nidrs::meta(nidrs::datasets::ServiceType::from("Controller"))]
        #[nidrs::meta(nidrs::datasets::ServiceName::from(#ident_name))]
        #[nidrs::meta(nidrs::datasets::ControllerPath::from(#path))]
        #[nidrs::macros::__controller_derive]
        #func
    })
}

#[proc_macro_attribute]
pub fn __controller_derive(args: TokenStream, input: TokenStream) -> TokenStream {
    // println!("__controller_derive {:?}", meta_parse::get_meta_value("METADATA:nidrs::datasets::ServiceName"));
    __service_derive(ServiceType::Controller, input)
}

fn __service_derive(service_type: ServiceType, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemStruct);

    println!("// service_derive {:?}", func.ident.to_string());

    let inject_tokens: TokenStream2 = gen_service_inject_tokens(service_type, &func);

    meta_parse::stash();

    TokenStream::from(quote! {
        #[derive(Default)]
        #func

        #inject_tokens
    })
}

#[proc_macro_attribute]
pub fn __route_derive(args: TokenStream, input: TokenStream) -> TokenStream {
    return route_derive(args, input);
}

#[proc_macro_attribute]
pub fn module(args: TokenStream, input: TokenStream) -> TokenStream {
    // 解析宏的参数
    let args = parse_macro_input!(args as ModuleArgs);
    let func = parse_macro_input!(input as ItemStruct);
    let ident = func.ident.clone();
    let ident_name = ident.to_string();

    let controller_register_tokens = gen_controller_register_tokens_v2(ident_name.clone(), args.controllers.clone());
    let service_register_tokens = gen_service_register_tokens(ident_name.clone(), args.services.clone());
    let defaults_interceptors = DEFAULT_INTERS
        .lock()
        .unwrap()
        .clone()
        .iter()
        .map(|inter| {
            return syn::Ident::new(inter, Span::call_site().into()).to_token_stream();
        })
        .collect::<Vec<TokenStream2>>();
    let interceptor_register_tokens = gen_interceptor_register_tokens(
        ident_name.clone(),
        utils::merge_vec(args.interceptors.clone(), utils::merge_vec(defaults_interceptors, inters_to_vec_tokens())),
    );
    let (import_names_tokens, imports_register_tokens) = gen_imports_register_tokens(ident_name.clone(), args.imports.clone());
    let imports_register_names = args.imports.clone().iter().map(|import_tokens| import_tokens.to_string()).collect::<Vec<String>>();
    let exports_names: Vec<String> = args.exports.clone().iter().map(|export_tokens| export_tokens.to_string()).collect::<Vec<String>>();
    let exports_names_tokens = exports_names
        .iter()
        .map(|export_name| {
            quote! {
                #export_name
            }
        })
        .collect::<Vec<TokenStream2>>();
    let exports_names_tokens = TokenStream2::from(quote! {
        Vec::from([#(#exports_names_tokens),*])
    });

    let services_dep_inject_tokens: TokenStream2 = gen_dep_inject_tokens("get_service", ident_name.clone(), args.services.clone());
    let controller_dep_inject_tokens = gen_dep_inject_tokens("get_controller", ident_name.clone(), args.controllers.clone());
    let interceptor_dep_inject_tokens = gen_dep_inject_tokens("get_interceptor", ident_name.clone(), args.interceptors.clone());

    let trigger_on_module_init_tokens: TokenStream2 = gen_events_trigger_tokens(ident_name.clone(), "on_module_init");
    let trigger_on_module_destroy_tokens = gen_events_trigger_tokens(ident_name.clone(), "on_module_destroy");

    let module_meta_tokens = cmeta::CMeta::build_tokens();
    let is_global_tokens = if let Some(MetaValue::Bool(bool)) = meta_parse::get_meta_value("Global") { bool } else { false };
    println!("// module {:?}", ident.to_string());

    CURRENT_CONTROLLER.lock().unwrap().take();
    ROUTES.lock().unwrap().clear();
    EVENTS.lock().unwrap().clear();
    INTERS.lock().unwrap().clear();
    meta_parse::clear();
    g_current_module::end_mod();

    return TokenStream::from(quote! {
        #[derive(Default)]
        #func

        impl nidrs::Module for #ident {
            fn init(self, mut ctx: nidrs::ModuleCtx) -> nidrs::ModuleCtx{
                use nidrs::{Service, Controller, Interceptor, InterCtx, InterceptorHandler, ModuleCtx, StateCtx, ImplMeta};
                if ctx.modules.contains_key(#ident_name) {
                    return ctx;
                }
                nidrs_macro::log!("Registering module {}.", #ident_name);
                ctx.modules.insert(#ident_name.to_string(), Box::new(self));
                ctx.imports.insert(#ident_name.to_string(), #import_names_tokens);
                // ctx.exports.insert(#module_name.to_string(), #exports_names_tokens);
                ctx.append_exports(#ident_name, #exports_names_tokens, #is_global_tokens);

                // {
                    #interceptor_register_tokens

                    #controller_register_tokens

                    #service_register_tokens
                // }
                // {
                    #imports_register_tokens
                // }
                // {
                    #services_dep_inject_tokens

                    #controller_dep_inject_tokens

                    #interceptor_dep_inject_tokens
                // }

                // {
                    #trigger_on_module_init_tokens
                // }

                ctx
            }

            fn destroy(&self, ctx: &nidrs::ModuleCtx){
                #trigger_on_module_destroy_tokens
                nidrs::log!("Destroying module {}.", #ident_name);
            }
        }

        impl nidrs::ImplMeta for #ident{
            fn __meta() -> nidrs::Meta {
                #module_meta_tokens
            }
        }
    });
}

#[proc_macro_attribute]
pub fn injectable(args: TokenStream, input: TokenStream) -> TokenStream {
    g_current_module::begin_mod();
    let func = parse_macro_input!(input as ItemStruct);
    let func_ident = func.ident.clone();
    let func_ident_name = func.ident.to_string();
    CURRENT_SERVICE.lock().unwrap().replace(ServiceMeta { name: func_ident_name.clone() });

    let call_site = Span::call_site();
    let binding = call_site.source_file().path();
    let call_site_str = binding.to_string_lossy();
    let call_site_line = call_site.start().line();

    // println!("// injectable {}", func.ident.to_string());
    import_path::push_path(&func_ident_name.clone());

    return TokenStream::from(quote! {
        #[nidrs::meta(nidrs::datasets::ServiceType::from("Service"))]
        #[nidrs::meta(nidrs::datasets::ServiceName::from(#func_ident_name))]
        #[nidrs::macros::__injectable_derive]
        #func
    });
}

#[proc_macro_attribute]
pub fn __injectable_derive(args: TokenStream, input: TokenStream) -> TokenStream {
    __service_derive(ServiceType::Service, input)
}

#[proc_macro_attribute]
pub fn interceptor(args: TokenStream, input: TokenStream) -> TokenStream {
    g_current_module::begin_mod();
    let func = parse_macro_input!(input as ItemStruct);
    let func_ident = func.ident.clone();
    let func_ident_name = func.ident.to_string();
    CURRENT_SERVICE.lock().unwrap().replace(ServiceMeta { name: func_ident_name.clone() });

    import_path::push_path(&func_ident_name.clone());

    return TokenStream::from(quote! {
        #[nidrs::meta(nidrs::datasets::ServiceType::from("Interceptor"))]
        #[nidrs::meta(nidrs::datasets::ServiceName::from(#func_ident_name))]
        #[nidrs::macros::__interceptor_derive]
        #func
    });
}

#[proc_macro_attribute]
pub fn __interceptor_derive(args: TokenStream, input: TokenStream) -> TokenStream {
    __service_derive(ServiceType::Interceptor, input)
}

#[proc_macro_attribute]
pub fn on_module_init(args: TokenStream, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemFn);

    let ident = func.sig.ident.clone();
    let current_service = CURRENT_SERVICE.lock().unwrap().clone();

    EVENTS.lock().unwrap().entry("on_module_init".to_string()).or_insert(vec![]).push((current_service.unwrap().name, ident.to_string()));

    return TokenStream::from(quote! {
        #func
    });
}

#[proc_macro_attribute]
pub fn on_module_destroy(args: TokenStream, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemFn);

    let ident = func.sig.ident.clone();
    let current_service = CURRENT_SERVICE.lock().unwrap().clone();

    EVENTS.lock().unwrap().entry("on_module_destroy".to_string()).or_insert(vec![]).push((current_service.unwrap().name, ident.to_string()));

    return TokenStream::from(quote! {
        #func
    });
}

#[proc_macro_attribute]
pub fn uses(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as ExprList);
    let input_type = input.clone();
    let input_type = parse_macro_input!(input_type as InterceptorArgs);
    let used_ident = input_type.ident;
    let inter_names = args
        .items
        .iter()
        .map(|arg| {
            if let Expr::Path(path) = arg {
                path.to_token_stream().to_string()
            } else {
                panic!("Invalid argument");
            }
        })
        .collect::<Vec<String>>();
    if let TokenType::Fn(_) = input_type.typ {
        let controller_name = CURRENT_CONTROLLER.lock().unwrap().as_ref().unwrap().name.clone();
        let hook_name = controller_name + ":" + &used_ident.to_string();
        INTERS.lock().unwrap().entry(hook_name).or_insert(vec![]).append(&mut inter_names.clone());
    } else if let TokenType::Struct(_) = input_type.typ {
        INTERS.lock().unwrap().entry(used_ident.to_string()).or_insert(vec![]).append(&mut inter_names.clone());
    }
    return input;
}

#[proc_macro_attribute]
pub fn default_uses(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as ExprList);
    let input_type = input.clone();
    let input_type = parse_macro_input!(input_type as InterceptorArgs);
    let used_ident = input_type.ident;
    let inter_names = args
        .items
        .iter()
        .map(|arg| {
            if let Expr::Path(path) = arg {
                path.to_token_stream().to_string()
            } else {
                panic!("Invalid argument");
            }
        })
        .collect::<Vec<String>>();

    DEFAULT_INTERS.lock().unwrap().append(&mut inter_names.clone());

    // println!("// default_uses {:?}", inter_names);

    return input;
}

#[proc_macro_attribute]
pub fn meta(args: TokenStream, input: TokenStream) -> TokenStream {
    let raw = input.clone();
    let fun = parse_macro_input!(input as InterceptorArgs);

    g_current_module::check_mod();

    let level = cmeta::CMeta::get_level();

    if let None = level {
        cmeta::init_app_meta();
        cmeta::init_module_meta();
    }

    let level = cmeta::CMeta::get_level();

    if let TokenType::Struct(item) = &fun.typ {
        let cur_mod = g_current_module::get();
        if let Some(cur_mod) = cur_mod {
            let level_mod = cmeta::CMeta::get_stack("module");
            if let Some(cmeta::CMetaValue::String(name)) = &level_mod {
                if &cur_mod.name != name {
                    let deep = cmeta::CMeta::get_deep();
                    let loop_deep = deep - 1;
                    for _ in 0..loop_deep {
                        cmeta::CMeta::pop();
                    }

                    cmeta::CMeta::push(cmeta::CMetaLevel::Module(cur_mod.name.clone()));
                    cmeta::CMeta::push(cmeta::CMetaLevel::Service(item.ident.to_string()));
                }
            }
        }

        let level = cmeta::CMeta::get_level();
        if let Some(cmeta::CMetaLevel::Service(name)) = level {
            if item.ident.to_string() != name {
                cmeta::CMeta::pop();
                cmeta::CMeta::push(cmeta::CMetaLevel::Service(item.ident.to_string()));
            }
        } else if let Some(cmeta::CMetaLevel::Handler(name)) = level {
            cmeta::CMeta::pop();
            cmeta::CMeta::pop();
            cmeta::CMeta::push(cmeta::CMetaLevel::Service(item.ident.to_string()));
        } else {
            cmeta::CMeta::push(cmeta::CMetaLevel::Service(item.ident.to_string()));
        }
    } else if let TokenType::Fn(item) = &fun.typ {
        if let Some(cmeta::CMetaLevel::Handler(name)) = level {
            if item.sig.ident.to_string() != name {
                cmeta::CMeta::pop();
                cmeta::CMeta::push(cmeta::CMetaLevel::Handler(item.sig.ident.to_string()));
            }
        } else {
            cmeta::CMeta::push(cmeta::CMetaLevel::Handler(item.sig.ident.to_string()));
        }
    }

    let targs = args.clone();
    let cmeta = parse_macro_input!(targs as cmeta::CMeta);
    cmeta::CMeta::collect(cmeta);

    let args = parse_macro_input!(args as MetaArgs);
    // println!("// meta {:?} {:?}", func.ident.to_string(), args.kv.keys());

    meta_parse::collect(args);

    return raw;
}

#[proc_macro]
pub fn throw(input: TokenStream) -> TokenStream {
    let input = TokenStream2::from(input);
    let call_site = Span::call_site();
    let binding = call_site.source_file().path();
    let call_site_str = binding.to_string_lossy();
    let call_site_line = call_site.start().line();

    // let binding = ROUTES.lock().unwrap();
    // let current_controller = CURRENT_CONTROLLER.lock().unwrap();
    // let struct_name =  current_controller.as_ref().unwrap().name.clone();
    // let method_name = binding.get(&struct_name).unwrap().keys().last().unwrap();

    // 构建返回的 TokenStream
    let expanded = quote! {
        // println!("Macro called from method: {}.{}", stringify!(#struct_name), stringify!(#method_name));
        // println!("Macro called from: {} line {}", #call_site_str, #call_site_line);
        nidrs::__throw(#input, &format!("from {} line {}", #call_site_str, #call_site_line))?;
    };

    expanded.into()
}

#[proc_macro]
pub fn log(input: TokenStream) -> TokenStream {
    let input = TokenStream2::from(input);

    let input_tokens = input.into_iter().collect::<Vec<_>>();

    return TokenStream::from(quote::quote! {
        print!("{} ", nidrs_extern::colored::Colorize::green("[nidrs]"));
        println!(#(#input_tokens)*);
    });
}

#[proc_macro]
pub fn elog(input: TokenStream) -> TokenStream {
    let input = TokenStream2::from(input);

    let input_tokens = input.into_iter().collect::<Vec<_>>();

    return TokenStream::from(quote::quote! {
        eprint!("{} ", nidrs_extern::colored::Colorize::red("[nidrs]"));
        eprintln!(#(#input_tokens)*);
    });
}

/// auto add a meta #[meta(version = String)]
#[proc_macro_attribute]
pub fn version(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as ExprList);
    let raw_input = TokenStream2::from(input.clone());

    let version = args
        .items
        .iter()
        .map(|arg| {
            if let Expr::Lit(lit) = arg {
                if let syn::Lit::Str(str) = lit.to_owned().lit {
                    str.value().trim().to_string()
                } else {
                    panic!("Invalid argument")
                }
            } else {
                panic!("Invalid argument")
            }
        })
        .collect::<Vec<String>>()
        .first()
        .unwrap()
        .clone();

    return TokenStream::from(quote! {
        #[nidrs::macros::meta(version = #version)]
        #raw_input
    });
}

#[proc_macro_attribute]
pub fn disable_default_prefix(args: TokenStream, input: TokenStream) -> TokenStream {
    let raw_input = TokenStream2::from(input.clone());

    return TokenStream::from(quote! {
        #[nidrs::macros::meta(nidrs::datasets::DisableDefaultPrefix::from(true))]
        #raw_input
    });
}

#[proc_macro_attribute]
pub fn global(args: TokenStream, input: TokenStream) -> TokenStream {
    let raw_input = TokenStream2::from(input.clone());

    return TokenStream::from(quote! {
        #[nidrs::macros::meta(nidrs::datasets::Global::from(true))]
        #raw_input
    });
}

#[proc_macro_attribute]
pub fn main(args: TokenStream, input: TokenStream) -> TokenStream {
    cmeta::CMeta::pop();
    let func = parse_macro_input!(input as ItemFn);
    let ident = func.sig.ident.clone();

    let import_mod_tokens = import_path::gen_import_mod_tokens();

    // println!("main {:?} {}", func.sig.ident.to_string(), import_mod_tokens.to_string());

    let main_tokens = TokenStream2::from(quote! {
        #func
        #import_mod_tokens
    });

    return main_tokens.into();
}

fn route(method: &str, args: TokenStream, input: TokenStream) -> TokenStream {
    let path: String = if args.is_empty() {
        "".to_string()
    } else {
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
        path
    };
    println!("// route {} {} {:?} {:?}", method, path, meta_parse::get_meta_value("controller_router_path"), meta_parse::get_meta_value("version"));
    let func = parse_macro_input!(input as ItemFn);

    let name = func.sig.ident.to_string();

    let vis = func.vis.clone();
    let ident = func.sig.ident.clone();
    let ident_name = ident.to_string();
    let mut pindex = 0;
    let mut is_body = false;
    let mut is_meta = false;

    let func_args = func
        .sig
        .inputs
        .iter()
        .map(|arg| match arg {
            FnArg::Typed(PatType { pat, ty, .. }) => {
                // let pat = pat.to_token_stream();
                let ty = ty.to_token_stream();
                if ty.to_string().contains("Json") {
                    is_body = true;
                } else if ty.to_string().contains("Meta") {
                    is_meta = true;
                }
                let pat = format!("p{}", pindex);
                let pat_indent = syn::Ident::new(&pat, Span::call_site().into());
                pindex += 1;
                quote! {
                    #pat_indent
                }
            }
            _ => quote! {},
        })
        .map(|arg| arg.to_string())
        .filter(|v| !v.is_empty())
        .collect::<Vec<String>>();

    let route = RouteMeta { method: method.to_string(), path: path.clone(), name: name.clone(), func_args, is_body, is_meta };

    let mut binding = ROUTES.lock().unwrap();
    let controller = binding.get_mut(&CURRENT_CONTROLLER.lock().unwrap().as_ref().unwrap().name).unwrap();
    controller.insert(name.clone(), route);

    TokenStream::from(quote! {
        #[nidrs::meta(nidrs::datasets::RouterName::from(#ident_name))]
        #[nidrs::meta(nidrs::datasets::RouterMethod::from(#method))]
        #[nidrs::meta(nidrs::datasets::RouterPath::from(#path))]
        #[nidrs::__route_derive]
        #func
    })
}

fn route_derive(args: TokenStream, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemFn);
    // let meta_tokens: TokenStream2 = meta_parse::build_tokens();
    meta_parse::clear_meta();
    let fn_ident = func.sig.ident.clone();
    let meta_fn_ident = syn::Ident::new(format!("__meta_{}", func.sig.ident.to_string()).as_str(), func.span().clone());

    println!("// route_derive {:?}", func.sig.ident.to_string());

    let route_fn_ident = syn::Ident::new(format!("__route_{}", func.sig.ident.to_string()).as_str(), func.span().clone());
    let route_fn_name = route_fn_ident.to_string();

    let route_method_name: String =
        cmeta::CMeta::get_stack_data("RouterMethod").expect(&format!("[route_derive] {} RouterMethod not found", route_fn_name));
    let route_method = syn::Ident::new(&route_method_name, func.span().clone());

    let meta_tokens = cmeta::CMeta::build_tokens();

    let mut pindex = 0;
    let mut body_token = quote! {};
    let mut meta_token = quote! {};
    let mut axum_args = vec![];
    let mut func_args = vec![];
    func.sig.inputs.iter().for_each(|arg| match arg {
        FnArg::Typed(PatType { pat, ty, .. }) => {
            // let pat = pat.to_token_stream();
            // println!(">> route_derive {:?} {:?}", pat.to_token_stream().to_string(), ty.to_token_stream().to_string());
            let pat = format!("p{}", pindex);
            let pat_ident = syn::Ident::new(&pat, Span::call_site().into());
            pindex += 1;
            let ty = ty.to_token_stream();
            if ty.to_string().contains("Json") {
                body_token = quote! {};
                axum_args.push(pat_ident.clone());
            } else if ty.to_string().contains("Meta") {
                let pat_ident_t = pat_ident.clone();
                meta_token = quote! {
                    let mut #pat_ident_t = nidrs::Meta::new();
                    #pat_ident_t.extend_ref(meta);
                };
            } else {
                axum_args.push(pat_ident.clone());
            }
            func_args.push(quote! {
                #pat_ident
            })
        }
        _ => (),
    });

    let func_args = TokenStream2::from(quote! {
        #(#func_args),*
    });
    let axum_args = TokenStream2::from(quote! {
        #(#axum_args),*
    });

    // println!(" route_derive {:?} {:?}", func.sig.ident.to_string(), func_args);

    TokenStream::from(quote! {
        #func

        pub fn #meta_fn_ident(&self)->nidrs::Meta{
            #meta_tokens
        }

        pub fn #route_fn_ident(&self, mut ctx: nidrs::ModuleCtx)->nidrs::ModuleCtx{
            use nidrs::externs::axum::{extract::Query, Json};
            use serde_json::Value;

            let mut meta = self.#meta_fn_ident();

            let router_info = ctx.get_router_info(&meta);

            if let Err(e) = router_info {
                panic!("[{}] {:?}", #route_fn_name,  e);
            }

            let full_path = router_info.unwrap();

            nidrs_macro::log!("Registering router '{} {}'.", #route_method_name.to_uppercase(), full_path);

            meta.set_data(nidrs::datasets::RouterFullPath(full_path.clone()));

            let module_name = meta.get::<&str>("module").unwrap();
            let controller_name = meta.get_data::<nidrs::datasets::ServiceName>().unwrap().value();

            let t_controller = ctx.get_controller::<Self>(module_name, controller_name);
            let meta = std::sync::Arc::new(meta);
            let t_meta = meta.clone();
            let router = nidrs::externs::axum::Router::new()
                .route(
                    &full_path,
                    // nidrs::externs::axum::routing::get(|state: nidrs::externs::axum::extract::State<nidrs::StateCtx>, req: nidrs::externs::axum::extract::Request| async move {
                    nidrs::externs::axum::routing::get(|#axum_args| async move {
                        #meta_token
                        t_controller.#fn_ident(#func_args).await
                        // return String::from("ok");
                    }),
                );
            ctx.routers
                .push(nidrs::RouterWrap {
                    router: router,
                    meta: t_meta,
                });

            ctx
        }

    })
}

fn gen_controller_register_tokens_v2(module_name: String, services: Vec<TokenStream2>) -> TokenStream2 {
    let binding = CURRENT_CONTROLLER.lock().unwrap();
    if let None = binding.as_ref() {
        return TokenStream2::new();
    }
    let current_controller = binding.as_ref().unwrap();
    let controller_path = current_controller.path.clone();
    let controller_tokens: Vec<TokenStream2>= services.iter().map(|controller_token| {
        let controller_name = controller_token.to_string();
        let binding = ROUTES.lock().unwrap();
        let controller = binding.get(&controller_name).unwrap();
        let controller_ident = syn::Ident::new(&controller_name, Span::call_site().into());
        let router_path = controller.iter().map(|(name, route)| {
            let route_ident = syn::Ident::new(&format!(
                "__route_{}",
                name
            ), Span::call_site().into());
       
            quote! {
                // {
                let t_controller = ctx.get_controller::<controller::#controller_ident>(#module_name, #controller_name);

                ctx = t_controller.#route_ident(ctx);
            }
        }).collect::<Vec<TokenStream2>>();

        quote! {
            if ctx.register_controller(#module_name, #controller_name, Box::new(std::sync::Arc::new(controller::#controller_ident::default()))) {
                #(#router_path)*
            }
        }
    }).collect::<Vec<TokenStream2>>();
    let controller_tokens = TokenStream2::from(quote! {
        #(#controller_tokens)*
    });
    return controller_tokens;
}

fn gen_controller_register_tokens(module_name: String, services: Vec<TokenStream2>) -> TokenStream2 {
    let binding = CURRENT_CONTROLLER.lock().unwrap();
    if let None = binding.as_ref() {
        return TokenStream2::new();
    }
    let current_controller = binding.as_ref().unwrap();
    let controller_path = current_controller.path.clone();
    let controller_tokens: Vec<TokenStream2>= services.iter().map(|controller_token| {
        let controller_name = controller_token.to_string();
        let binding = ROUTES.lock().unwrap();
        let controller = binding.get(&controller_name).unwrap();
        let controller_ident = syn::Ident::new(&controller_name, Span::call_site().into());
        let router_path = controller.iter().map(|(name, route)| {
            let method = route.method.clone();
            let method_ident = syn::Ident::new(&method, Span::call_site().into());
            let path = controller_path.clone() + &route.path.clone();
            let route_name = syn::Ident::new(&route.name, Span::call_site().into());
            // println!("route {} {:?}", route_name.to_string(), route.func_args);
            
            let (method_meta_tokens, arc_meta_tokens) = gen_controller_meta_tokens(&route_name);
            
            let (def_inter_tokens, handler) = gen_handler_tokens(&module_name, route, &controller_name, route_name);
    
            quote! {
                {
                let t_controller = ctx.get_controller::<controller::#controller_ident>(#module_name, #controller_name);

                #def_inter_tokens

                #method_meta_tokens

                let version = *meta.get::<&str>("version").unwrap_or(&ctx.defaults.default_version);
                let disable_default_prefix = meta.get_data::<nidrs::datasets::DisableDefaultPrefix>().unwrap_or(&nidrs::datasets::DisableDefaultPrefix(false)).value();
                let path = if disable_default_prefix { #path.to_string() } else { nidrs::template_format(&format!("{}{}", ctx.defaults.default_prefix, #path), [("version", version)]) };
                nidrs_macro::log!("Registering router '{} {}'.", #method.to_uppercase(), path);
                
                meta.set_data(nidrs::datasets::RouterFullPath(path.clone()));

                #arc_meta_tokens

                let route_meta = meta.clone();

                let router = nidrs::externs::axum::Router::new().route(
                    &path,
                    nidrs::externs::axum::routing::#method_ident(#handler),
                );
                ctx.routers.push(nidrs::RouterWrap{ router: router, meta: route_meta.clone() });
                }
            }
        }).collect::<Vec<TokenStream2>>();

        quote! {
            if ctx.register_controller(#module_name, #controller_name, Box::new(std::sync::Arc::new(controller::#controller_ident::default()))) {
                #(#router_path)*
            }
        }
    }).collect::<Vec<TokenStream2>>();
    let controller_tokens = TokenStream2::from(quote! {
        #(#controller_tokens)*
    });
    return controller_tokens;
}

fn gen_handler_tokens(module_name: &str, route: &RouteMeta, controller_name: &String, route_name: syn::Ident) -> (TokenStream2, TokenStream2) {
    let (func_args, inter_ids, def_inter_tokens, def_clone_inter_tokens) =
        gen_controller_interceptor(module_name, route, controller_name, &route_name);

    // meta handle

    let handler = if inter_ids.is_empty() {
        let mut def_func_args: Vec<String> = route.func_args.clone();
        let meta_tokens = if route.is_meta {
            // 移除第一个
            def_func_args.remove(0);
            quote! {
                let p0 = t_meta;
            }
        } else {
            quote! {}
        };
        let def_func_args = str_args_to_indent(def_func_args);
        quote! {
            |#def_func_args| async move {
                let mut t_meta = nidrs::Meta::new();
                t_meta.extend_ref(meta);
                #meta_tokens
                t_controller.#route_name(#func_args).await
            }
        }
    } else {
        let mut def_func_args = route.func_args.clone();
        if route.is_meta {
            // 移除第一个
            def_func_args.remove(0);
        }
        let def_func_args = str_args_to_indent(def_func_args);
        quote! {
            |parts, #def_func_args| async move {
                let mut t_meta = nidrs::Meta::new();
                t_meta.extend_ref(meta);
                #def_clone_inter_tokens
            }
        }
    };
    (def_inter_tokens, handler)
}

fn gen_controller_interceptor(
    module_name: &str,
    route: &RouteMeta,
    controller_name: &String,
    route_name: &syn::Ident,
) -> (TokenStream2, Vec<(TokenStream2, TokenStream2)>, TokenStream2, TokenStream2) {
    let func_args = str_args_to_indent(route.func_args.clone());
    let noop_ids: Vec<String> = vec![];
    let inter_name = controller_name.clone() + ":" + &route_name.to_string();
    let binding = INTERS.lock().unwrap();
    let struct_inter_ids = binding.get(controller_name).unwrap_or(&noop_ids);
    let default_inter_ids: std::sync::MutexGuard<Vec<String>> = DEFAULT_INTERS.lock().unwrap();
    let struct_inter_ids: Vec<String> = struct_inter_ids.iter().chain(default_inter_ids.iter()).map(|v| v.clone()).collect();
    let inter_ids = binding.get(&inter_name).unwrap_or(&noop_ids);
    let inter_ids = struct_inter_ids.iter().chain(inter_ids.iter()).collect::<Vec<&String>>();

    // interceptor handle
    let inter_ids = inter_ids
        .iter()
        .map(|inter_id| {
            let crate_tokens = import_path::gen_import_tokens(&inter_id);
            let inter_id = syn::Ident::new(inter_id, Span::call_site().into()).to_token_stream();
            (inter_id, crate_tokens)
        })
        .collect::<Vec<(TokenStream2, TokenStream2)>>();
    let mut i = 0;
    let inter_tokens = inter_ids
        .iter()
        .map(|inter_wrap| {
            let inter_import = inter_wrap.1.clone();
            let inter_id = inter_wrap.0.clone();
            let inter_name = inter_id.to_string();
            let prev_t_interceptor_ident = syn::Ident::new(format!("t_interceptor_{}", i.to_string()).as_str(), Span::call_site().into());
            let prev_t_inter_fn_indent = syn::Ident::new(format!("t_inter_fn_{}", i.to_string()).as_str(), Span::call_site().into());
            i += 1;
            let t_interceptor_ident = syn::Ident::new(format!("t_interceptor_{}", i.to_string()).as_str(), Span::call_site().into());
            let t_inter_fn_indent = syn::Ident::new(format!("t_inter_fn_{}", i.to_string()).as_str(), Span::call_site().into());

            let mut tokens: Vec<TokenStream2> = vec![];
            if i == 1 {
                let mut t_vec = route.func_args.clone();
                let body_tokens = if route.is_body {
                    t_vec.pop();
                    t_vec.push("t_body".to_string());
                    quote! {
                        let t_body = ctx.body;
                    }
                } else {
                    quote! {}
                };
                let meta_tokens = if route.is_meta {
                    quote! {
                        let p0 = ctx.meta;
                    }
                } else {
                    quote! {}
                };
                let func_args = str_args_to_indent(t_vec);
                tokens.push(quote! {
                    let #prev_t_inter_fn_indent = |ctx: InterCtx<_>| async move {
                        #body_tokens
                        #meta_tokens
                        t_controller.#route_name(#func_args).await
                    };
                });
            }
            if i == inter_ids.len() {
                tokens.push(quote! {
                    #prev_t_interceptor_ident.interceptor(ctx, #prev_t_inter_fn_indent).await
                });
            } else {
                tokens.push(quote! {
                    let #t_inter_fn_indent = |ctx: InterCtx<_>| async move {
                        #prev_t_interceptor_ident.interceptor(ctx, #prev_t_inter_fn_indent).await
                    };
                });
            }
            (
                quote! {
                    let #prev_t_interceptor_ident = ctx.get_interceptor::<#inter_import>(#module_name, #inter_name);

                    // let #prev_t_interceptor_ident = ctx.interceptors.get(stringify!(#inter_id)).unwrap();
                    // let #prev_t_interceptor_ident = #prev_t_interceptor_ident.downcast_ref::<std::sync::Arc<#inter_import>>().unwrap();
                    // let #prev_t_interceptor_ident = #prev_t_interceptor_ident.clone();
                },
                quote! {
                    #(#tokens)*
                },
                quote! {},
                quote! {},
            )
        })
        .collect::<Vec<(TokenStream2, TokenStream2, TokenStream2, TokenStream2)>>();
    let def_inter_tokens = inter_tokens.iter().map(|(tokens, _, _, _)| tokens.clone()).collect::<Vec<TokenStream2>>();
    let def_inter_tokens = TokenStream2::from(quote! {
        #(#def_inter_tokens)*
    });
    let def_clone_inter_tokens = inter_tokens.iter().map(|(_, tokens, _, _)| tokens.clone()).collect::<Vec<TokenStream2>>();
    let ctx_body_tokens = if route.is_body {
        let last_arg_indent = syn::Ident::new(route.func_args.last().unwrap().as_str(), Span::call_site().into());
        quote! {
            let t_body = #last_arg_indent;
        }
    } else {
        quote! {
            let t_body = nidrs_extern::axum::body::Body::empty();
        }
    };
    let def_clone_inter_tokens = TokenStream2::from(quote! {
        #ctx_body_tokens
        let ctx = InterCtx {
            meta: t_meta,
            parts,
            body: t_body,
        };
        #(#def_clone_inter_tokens)*
    });
    (func_args, inter_ids, def_inter_tokens, def_clone_inter_tokens)
}

fn gen_controller_meta_tokens(route_name: &syn::Ident) -> (TokenStream2, TokenStream2) {
    let struct_meta_tokens: TokenStream2 = quote! {
        let mut meta = nidrs::get_meta(t_controller.clone());
    };
    let method_meta = syn::Ident::new(format!("__meta_{}", route_name).as_str(), Span::call_site().into());
    let method_meta_tokens = quote! {
        let t_meta = t_controller.#method_meta();
        meta.merge(t_meta);
    };
    let meta_tokens = TokenStream2::from(quote! {
        let meta = std::sync::Arc::new(meta);
    });
    let struct_meta_tokens = TokenStream2::from(quote! {
        #struct_meta_tokens
        #method_meta_tokens
    });
    (struct_meta_tokens, meta_tokens)
}

fn gen_service_register_tokens(module_name: String, services: Vec<TokenStream2>) -> TokenStream2 {
    let service_tokens = services
        .iter()
        .map(|service_tokens| {
            let service_name = service_tokens.to_string();
            let service_ident = service_tokens;

            quote! {
                let svc = std::sync::Arc::new(#service_ident::default());
                // #register_global_tokens
                ctx.register_service(#module_name, #service_name, Box::new(svc));
            }
        })
        .collect::<Vec<TokenStream2>>();
    let service_tokens = TokenStream2::from(quote! {
        #(#service_tokens)*
    });
    return service_tokens;
}

fn gen_interceptor_register_tokens(module_name: String, services: Vec<TokenStream2>) -> TokenStream2 {
    // println!("// gen_interceptor_register_tokens {} {:?}", module_name, services);
    let interceptor_tokens = services
        .iter()
        .map(|interceptor_token| {
            let interceptor_name = interceptor_token.to_string();
            let interceptor_ident = interceptor_token;
            let import_interceptor_tokens = import_path::gen_import_tokens(&interceptor_name);
            quote! {
                ctx.register_interceptor(#module_name, #interceptor_name, Box::new(std::sync::Arc::new(#import_interceptor_tokens::default())));
            }
        })
        .collect::<Vec<TokenStream2>>();
    let interceptor_tokens = TokenStream2::from(quote! {
        #(#interceptor_tokens)*
    });
    return interceptor_tokens;
}

fn gen_imports_register_tokens(module_name: String, imports: Vec<TokenStream2>) -> (TokenStream2, TokenStream2) {
    let mut import_names = vec![];
    let imports = imports
        .iter()
        .map(|import_tokens| {
            let import_name = import_tokens.to_string();

            let import_call = syn::parse2::<ExprCall>(import_tokens.clone());
            if let Ok(import_call) = import_call {
                if let Expr::Path(path) = import_call.func.as_ref() {
                    let module_ident = path.path.segments.first().unwrap().ident.clone();
                    import_names.push(module_ident.to_string());
                    let dyn_module_name = module_ident.to_string();
                    quote! {
                        let dyn_module = #import_call;
                        let mut dyn_module_services = dyn_module.services;
                        dyn_module_services.drain().for_each(|(k, v)| {
                            ctx.register_service(#dyn_module_name, k, v);
                        });
                        let mut dyn_module_exports = dyn_module.exports;
                        ctx.append_exports(#dyn_module_name, dyn_module_exports, nidrs::get_meta_by_type::<#module_ident>().get_data::<nidrs::datasets::Global>().unwrap_or(&nidrs::datasets::Global(false)).value());
                        let mut ctx = #module_ident::default().init(ctx);
                    }
                } else {
                    panic!("Invalid import.")
                }
            } else {
                import_names.push(import_name.clone());
                quote! {
                    let mut ctx = #import_tokens::default().init(ctx);
                }
            }
        })
        .collect::<Vec<TokenStream2>>();

    let imports = TokenStream2::from(quote! {
        #(#imports)*
    });
    let import_names = import_names
        .iter()
        .map(|import_name| {
            let import_name = import_name.to_string();
            quote! {
                #import_name.to_string()
            }
        })
        .collect::<Vec<TokenStream2>>();
    let import_names = TokenStream2::from(quote! {
        Vec::from([#(#import_names),*])
    });

    return (import_names, imports);
}

fn gen_dep_inject_tokens(con: &str, module_name: String, services: Vec<TokenStream2>) -> TokenStream2 {
    let con_ident = syn::Ident::new(con, Span::call_site().into());
    let controller_tokens = services
        .iter()
        .map(|tokens| {
            let controller_name = tokens.to_string();
            let controller_ident = tokens;

            quote! {
                let t = ctx.#con_ident::<#controller_ident>(#module_name, #controller_name);
                nidrs_macro::log!("Injecting {}::{}.", #module_name, #controller_name);
                let ctx = t.inject(ctx, &#module_name);
            }
        })
        .collect::<Vec<TokenStream2>>();
    let controller_tokens = TokenStream2::from(quote! {
        #(#controller_tokens)*
    });
    return controller_tokens;
}

fn gen_service_inject_tokens(service_type: ServiceType, func: &ItemStruct) -> TokenStream2 {
    let is_service = service_type == ServiceType::Service;
    let service_type_indent = syn::Ident::new(service_type.into(), Span::call_site().into());
    let service_name_ident = func.ident.clone();

    let fields: Vec<TokenStream2> = if let syn::Fields::Named(fields) = &func.fields {
        fields
            .named
            .iter()
            .map(|field| {
                let field_ident = field.ident.as_ref().unwrap();
                let field_type = &field.ty;

                if let Type::Path(type_path) = field_type {
                    let type_ident = type_path.path.segments.first().unwrap().ident.to_string();
                    if type_ident == "Inject" {
                        let type_args = type_path.path.segments.first().unwrap().arguments.to_owned();
                        if let syn::PathArguments::AngleBracketed(args) = type_args {
                            let type_arg = args.args.first().unwrap();
                            if let syn::GenericArgument::Type(ty) = type_arg {
                                let injected_type = ty.to_token_stream();
                                let injected_type_name = injected_type.to_string();
                                let con_ident = match service_type {
                                    ServiceType::Service => syn::Ident::new("get_service", Span::call_site().into()),
                                    ServiceType::Controller => syn::Ident::new("get_controller", Span::call_site().into()),
                                    ServiceType::Interceptor => syn::Ident::new("get_interceptor", Span::call_site().into()),
                                };
                                quote! {
                                    let service = ctx.get_service::<#injected_type>(&module_name, #injected_type_name);
                                    self.#field_ident.inject(service.clone());
                                }
                            } else {
                                quote! {}
                            }
                        } else {
                            quote! {}
                        }
                    } else {
                        quote! {}
                    }
                } else {
                    quote! {}
                }
            })
            .collect::<Vec<TokenStream2>>()
    } else {
        vec![]
    };
    let middle_tokens = if is_service {
        quote! {}
    } else {
        quote! {
            impl nidrs::#service_type_indent for #service_name_ident {}
        }
    };

    let meta_tokens = cmeta::CMeta::build_tokens();

    let inject_tokens = TokenStream2::from(quote! {
        #middle_tokens
        impl nidrs::Service for #service_name_ident {
            fn inject(&self, ctx: nidrs::ModuleCtx, module_name: &str) -> nidrs::ModuleCtx{
                #(#fields)*
                ctx
            }
        }

        impl nidrs::ImplMeta for #service_name_ident{
            fn __meta() -> nidrs::Meta {
                #meta_tokens
            }
        }
    });

    return inject_tokens;
}

fn gen_events_trigger_tokens(module_name: String, event_name: &str) -> TokenStream2 {
    // let event_name_ident = syn::Ident::new(event_name, Span::call_site().into());
    let binding = EVENTS.lock().unwrap();
    let on_module_event = binding.get(event_name);
    if let None = on_module_event {
        return TokenStream2::new();
    }
    let events_trigger_tokens = on_module_event
        .unwrap()
        .iter()
        .map(|(service_name, func)| {
            let service_ident = syn::Ident::new(service_name, Span::call_site().into());
            let func_ident = syn::Ident::new(func, Span::call_site().into());
            quote! {

                let service = ctx.get_service::<#service_ident>(#module_name, #service_name);
                nidrs_macro::log!("Triggering event {} for {}::{}.", #event_name, #module_name, #service_name);
                service.#func_ident();
            }
        })
        .collect::<Vec<TokenStream2>>();
    let events_trigger_tokens = TokenStream2::from(quote! {
        #(#events_trigger_tokens)*
    });
    return events_trigger_tokens;
}

fn str_args_to_indent(args: Vec<String>) -> TokenStream2 {
    let args = args
        .iter()
        .map(|arg| {
            let arg_ident = syn::Ident::new(arg, Span::call_site().into());
            quote! {
                #arg_ident
            }
        })
        .collect::<Vec<TokenStream2>>();
    let args = TokenStream2::from(quote! {
        #(#args),*
    });
    return args;
}

fn inters_to_vec_tokens() -> Vec<TokenStream2> {
    let mut all_inters = Vec::new();
    let inters = INTERS.lock().unwrap();
    inters.iter().for_each(|(k, v)| {
        all_inters.extend(v.clone());
    });
    // 去重复
    all_inters.sort();
    all_inters.dedup();
    // println!("inters {:?}", all_inters);
    let inters = all_inters
        .iter()
        .map(|inter| {
            let inter_ident = syn::Ident::new(inter, Span::call_site().into());
            return inter_ident.to_token_stream();
        })
        .collect::<Vec<TokenStream2>>();
    inters
}
