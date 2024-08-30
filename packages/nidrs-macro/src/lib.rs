#![allow(warnings, unused)]
#![feature(proc_macro_span)]
extern crate proc_macro;

use std::{
    any::Any,
    borrow::BorrowMut,
    cell::RefCell,
    collections::HashMap,
    f64::consts::E,
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
use syn_args::{def, SynArgs};
use utils::merge_uses;
mod global;

use crate::meta_parse::MetaValue;

mod app_parse;
mod args;
mod cmeta;
mod current_module;
mod import_path;
mod meta_parse;
mod utils;

static ROUTES: Lazy<Mutex<HashMap<String, Vec<String>>>> = Lazy::new(|| Mutex::new(HashMap::new())); // HashMap<ControllerName, Vec<RouteName>>
static EVENTS: Lazy<Mutex<HashMap<String, Vec<(String, String)>>>> = Lazy::new(|| Mutex::new(HashMap::new())); // HashMap<EventName, Vec<(ServiceName,FName)>>
static DEFAULT_INTERS: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(vec![]));

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

#[syn_args::derive::declare(def::Option<def::String>)]
#[syn_args::derive::proc_attribute]
pub fn controller(args: Args, input: TokenStream) -> TokenStream {
    let path = match args {
        Args::F1(def::Option(Some(v))) => v.to_string(),
        _ => "".to_string(),
    };

    current_module::begin_mod();

    let func = parse_macro_input!(input as ItemStruct);

    let ident = func.ident.clone();
    let ident_name = ident.to_string();

    import_path::push_path(&func.ident.to_string());

    println!("// controller {} {:?}", ident.to_string(), func.attrs);
    ROUTES.lock().unwrap().insert(ident.to_string(), Vec::new());

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

#[syn_args::derive::declare(args::ModuleOptions)]
#[syn_args::derive::proc_attribute]
pub fn module(args: Args, input: TokenStream) -> TokenStream {
    // 解析宏的参数
    let module_options: args::ModuleOptions = {
        if let Args::F1(options) = args {
            options
        } else {
            panic!("Invalid argument");
        }
    };

    let func = parse_macro_input!(input as ItemStruct);
    let ident = func.ident.clone();
    let ident_name = ident.to_string();

    let controller_register_tokens = expand_controller_register(ident_name.clone(), &module_options.controllers);
    let service_register_tokens = expand_service_register(ident_name.clone(), &module_options.services);

    let all_interceptors = merge_defaults_interceptors(module_options.interceptors.clone());
    let interceptor_register_tokens = expand_interceptor_register(ident_name.clone(), &all_interceptors);
    let (import_names_tokens, imports_register_tokens) = expand_imports_register(ident_name.clone(), &module_options.imports);
    // let imports_register_names = args.imports.clone().iter().map(|import_tokens| import_tokens.to_string()).collect::<Vec<String>>();
    let exports_names_tokens = expand_exports_append(&module_options.exports);

    let services_dep_inject_tokens: TokenStream2 = expand_dep_inject("get_service", ident_name.clone(), &module_options.services);
    let controller_dep_inject_tokens = expand_dep_inject("get_controller", ident_name.clone(), &module_options.controllers);
    let interceptor_dep_inject_tokens = expand_dep_inject("get_interceptor", ident_name.clone(), &module_options.interceptors);

    let trigger_on_module_init_tokens: TokenStream2 = expand_events_trigger(ident_name.clone(), "on_module_init");
    let trigger_on_module_destroy_tokens = expand_events_trigger(ident_name.clone(), "on_module_destroy");

    let module_meta_tokens = cmeta::CMeta::build_tokens();
    let is_global_tokens = if let Some(MetaValue::Bool(bool)) = meta_parse::get_meta_value("Global") { bool } else { false };
    println!("// module {:?}", ident.to_string());

    ROUTES.lock().unwrap().clear();
    EVENTS.lock().unwrap().clear();
    meta_parse::clear();
    current_module::end_mod();

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
            fn __meta() -> nidrs::InnerMeta {
                #module_meta_tokens
            }
        }
    });
}

fn expand_exports_append(exports: &def::Array<def::Expr>) -> TokenStream2 {
    let exports_names: Vec<String> = exports.iter().map(|export_tokens| export_tokens.to_string()).collect::<Vec<String>>();
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
    exports_names_tokens
}

fn merge_defaults_interceptors(interceptors: def::Array<def::Expr>) -> def::Array<def::Expr> {
    let defaults_interceptors = def::Array(
        DEFAULT_INTERS
            .lock()
            .unwrap()
            .clone()
            .iter()
            .map(|inter| {
                return inter.as_str().into();
            })
            .collect::<Vec<def::Expr>>(),
    );

    let all_interceptors = defaults_interceptors.merge(interceptors);
    all_interceptors
}

#[proc_macro_attribute]
pub fn injectable(args: TokenStream, input: TokenStream) -> TokenStream {
    current_module::begin_mod();
    let func = parse_macro_input!(input as ItemStruct);
    let func_ident = func.ident.clone();
    let func_ident_name = func.ident.to_string();

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
    current_module::begin_mod();
    let func = parse_macro_input!(input as ItemStruct);
    let func_ident = func.ident.clone();
    let func_ident_name = func.ident.to_string();

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
    let name = ident.to_string();

    let current_service_name: String =
        cmeta::CMeta::get_stack_data("ServiceName").expect(&format!("[on_module_init] {} ServiceName not found", name));

    EVENTS.lock().unwrap().entry("on_module_init".to_string()).or_insert(vec![]).push((current_service_name, name));

    return TokenStream::from(quote! {
        #func
    });
}

#[proc_macro_attribute]
pub fn on_module_destroy(args: TokenStream, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemFn);

    let ident = func.sig.ident.clone();
    let name = ident.to_string();

    let current_service_name: String =
        cmeta::CMeta::get_stack_data("ServiceName").expect(&format!("[on_module_init] {} ServiceName not found", name));

    EVENTS.lock().unwrap().entry("on_module_destroy".to_string()).or_insert(vec![]).push((current_service_name, name));

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

    let expand = if let TokenType::Fn(item) = input_type.typ {
        quote! {
            #[nidrs::meta(method_uses = [#(#inter_names),*])]
        }
    } else if let TokenType::Struct(item) = input_type.typ {
        quote! {
            #[nidrs::meta(service_uses = [#(#inter_names),*])]
        }
    } else {
        panic!("Invalid argument");
    };
    println!("// uses {:?}", inter_names);
    let input = TokenStream2::from(input);
    return quote! {
        #expand
        #input
    }
    .into();
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

    current_module::check_mod();

    let level = cmeta::CMeta::get_level();

    if let None = level {
        cmeta::init_app_meta();
        cmeta::init_module_meta();
    }

    let level = cmeta::CMeta::get_level();

    if let TokenType::Struct(item) = &fun.typ {
        let cur_mod = current_module::get();
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

#[syn_args::derive::declare(def::String)]
#[syn_args::derive::proc_attribute]
pub fn version(args: Args, input: TokenStream) -> TokenStream {
    let version = match args {
        Args::F1(v) => v.to_string(),
        _ => "".to_string(),
    };

    let input = TokenStream2::from(input);
    return TokenStream::from(quote! {
        #[nidrs::macros::meta(version = #version)]
        #input
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

    let ident = func.sig.ident.clone();
    let ident_name = ident.to_string();

    let current_controller_name: String =
        cmeta::CMeta::get_stack_data("ServiceName").expect(&format!("[route] {} ServiceName not found", ident_name));

    let mut routes = ROUTES.lock().unwrap();
    let controller = routes.get_mut(&current_controller_name).unwrap();
    controller.push(ident_name.clone());

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
            // } else if ty.to_string().contains("InnerMeta") {
            //     let pat_ident_t = pat_ident.clone();
            //     meta_token = quote! {
            //         let mut #pat_ident_t = nidrs::InnerMeta::new();
            //         #pat_ident_t.extend_ref(t_meta);
            //     };
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

    let service_uses = merge_uses(["method_uses", "service_uses"]);

    let interceptor_uses_expand = service_uses
        .iter()
        .map(|inter| {
            let inter_ident = syn::Ident::new(inter, Span::call_site().into());

            quote! {
                .layer(axum::middleware::from_fn({
                    let inter = ctx.get_interceptor::<#inter_ident>(module_name, #inter);
                    move |req: axum::extract::Request, next: axum::middleware::Next| {
                        let inter = std::sync::Arc::clone(&inter);
                        async move {
                            let res = inter.intercept(req, next).await;
                            if let Ok(res) = res {
                                Ok(res.into_response())
                            } else {
                                Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
                            }
                        }
                    }
                }))
            }
        })
        .collect::<Vec<TokenStream2>>();

    // println!(" route_derive {:?} {:?}", func.sig.ident.to_string(), func_args);

    TokenStream::from(quote! {
        #func

        pub fn #meta_fn_ident(&self)->nidrs::InnerMeta{
            #meta_tokens
        }

        pub fn #route_fn_ident(&self, mut ctx: nidrs::ModuleCtx)->nidrs::ModuleCtx{
            use nidrs::externs::axum;
            use axum::response::IntoResponse;
            use nidrs::externs::axum::{extract::Query, Json};
            use nidrs::externs::meta::{InnerMeta, Meta};
            use nidrs::Interceptor;
            use serde_json::Value;

            let mut meta = self.#meta_fn_ident();

            let router_info = ctx.get_router_full(&meta);

            if let Err(e) = router_info {
                panic!("[{}] {:?}", #route_fn_name,  e);
            }

            let full_path = router_info.unwrap();

            nidrs_macro::log!("Registering router '{} {}'.", #route_method_name.to_uppercase(), full_path);

            meta.set_data(nidrs::datasets::RouterFullPath(full_path.clone()));

            let meta = Meta::new(meta);
            let module_name = meta.get::<&str>("module").unwrap();
            let controller_name = meta.get_data::<nidrs::datasets::ServiceName>().unwrap().value();

            let t_controller = ctx.get_controller::<Self>(module_name, controller_name);
            // let t_meta = meta.clone();
            let router = nidrs::externs::axum::Router::new()
                .route(
                    &full_path,
                    // nidrs::externs::axum::routing::get(|state: nidrs::externs::axum::extract::State<nidrs::StateCtx>, req: nidrs::externs::axum::extract::Request| async move {
                    nidrs::externs::axum::routing::get(|#axum_args| async move {
                        #meta_token
                        t_controller.#fn_ident(#func_args).await
                        // return String::from("ok");
                    }),
                )
                .layer(nidrs::externs::axum::Extension(meta.clone()))
                #(#interceptor_uses_expand)*;
                ;
            ctx.routers
                .push(nidrs::RouterWrap::new(router, meta));

            ctx
        }

    })
}

fn expand_controller_register(module_name: String, services: &def::Array<def::Expr>) -> TokenStream2 {
    let controller_tokens: Vec<TokenStream2> = services
        .iter()
        .map(|controller_token| {
            let controller_name = controller_token.to_string();
            let binding = ROUTES.lock().unwrap();
            let controller: &Vec<String> = binding.get(&controller_name).unwrap();
            let controller_ident = syn::Ident::new(&controller_name, Span::call_site().into());
            let router_path = controller
                .iter()
                .map(|name| {
                    let route_ident = syn::Ident::new(&format!("__route_{}", name), Span::call_site().into());

                    quote! {
                        // {
                        let t_controller = ctx.get_controller::<controller::#controller_ident>(#module_name, #controller_name);

                        ctx = t_controller.#route_ident(ctx);
                    }
                })
                .collect::<Vec<TokenStream2>>();

            quote! {
                if ctx.register_controller(#module_name, #controller_name, Box::new(std::sync::Arc::new(controller::#controller_ident::default()))) {
                    #(#router_path)*
                }
            }
        })
        .collect::<Vec<TokenStream2>>();
    let controller_tokens = TokenStream2::from(quote! {
        #(#controller_tokens)*
    });
    return controller_tokens;
}

fn expand_service_register(module_name: String, services: &def::Array<def::Expr>) -> TokenStream2 {
    let service_tokens = services
        .iter()
        .map(|service_tokens| {
            let service_name = service_tokens.to_string();
            let service_ident = service_tokens.to_token_stream();

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

fn expand_interceptor_register(module_name: String, services: &def::Array<def::Expr>) -> TokenStream2 {
    // println!("// gen_interceptor_register_tokens {} {:?}", module_name, services);
    let interceptor_tokens = services
        .iter()
        .map(|interceptor_token| {
            let interceptor_name = interceptor_token.to_string();
            let interceptor_ident = interceptor_token.to_token_stream();
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

fn expand_imports_register(module_name: String, imports: &def::Array<def::Expr>) -> (TokenStream2, TokenStream2) {
    let mut import_names = vec![];
    let imports = imports
        .iter()
        .map(|import_tokens| {
            let import_name = import_tokens.to_string();
            let import_tokens = import_tokens.to_token_stream();

            let import_call = syn::parse2::<ExprCall>(import_tokens.to_token_stream());
            if let Ok(import_call) = import_call {
                if let Expr::Path(path) = import_call.func.as_ref() {
                    let module_ident = path.path.segments.first().unwrap().ident.clone();
                    import_names.push(module_ident.to_string());
                    let dyn_module_name = module_ident.to_string();
                    quote! {
                        let dyn_module = #import_call;
                        let mut dyn_module_services = dyn_module.services;
                        dyn_module_services.drain().for_each(|(k, v)| {
                            ctx.register_service(#dyn_module_name, &k, v);
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

fn expand_dep_inject(con: &str, module_name: String, services: &def::Array<def::Expr>) -> TokenStream2 {
    let con_ident = syn::Ident::new(con, Span::call_site().into());
    let controller_tokens = services
        .iter()
        .map(|tokens| {
            let controller_name = tokens.to_string();
            let controller_ident = tokens.to_token_stream();

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
    let is_interceptor = service_type == ServiceType::Interceptor;
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
    let middle_tokens = if is_service || is_interceptor {
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
            fn __meta() -> nidrs::InnerMeta {
                #meta_tokens
            }
        }
    });

    return inject_tokens;
}

fn expand_events_trigger(module_name: String, event_name: &str) -> TokenStream2 {
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
