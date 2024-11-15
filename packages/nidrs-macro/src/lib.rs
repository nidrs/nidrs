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
use nidrs_extern::{datasets::ServiceType, helper::merge_derives};
use once_cell::sync::Lazy;
use proc_macro::{Ident, Span, TokenStream};
use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::{Punct, TokenTree};
use quote::{quote, ToTokens};
use syn::{
    meta,
    parse::{Parse, ParseStream},
    parse_str, Expr, ExprArray, ItemStruct, PatPath, Stmt, Token,
};
use syn::{parse, punctuated::Punctuated};
use syn::{parse_macro_input, spanned::Spanned, ItemFn};

mod args_parse;
use args_parse::*;
use syn_args::{def, SynArgs};
use utils::merge_uses;

mod app_parse;
mod args;
mod cmeta;
mod current_module;
mod impl_expand;
mod import_path;
mod utils;

static ROUTES: Lazy<Mutex<HashMap<String, Vec<String>>>> = Lazy::new(|| Mutex::new(HashMap::new())); // HashMap<ControllerName, Vec<RouteName>>
static EVENTS: Lazy<Mutex<HashMap<String, Vec<(String, String)>>>> = Lazy::new(|| Mutex::new(HashMap::new())); // HashMap<EventName, Vec<(ServiceName,FName)>>
static DEFAULT_INTERS: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(vec![]));

// #[proc_macro_attribute]
// pub fn test(args: TokenStream, input: TokenStream) -> TokenStream {
//     let input2 = TokenStream2::from(input.clone());
//     let func = parse_macro_input!(input as UFnStruct);
//     println!("test {}", func.ident.to_string());
//     return TokenStream::from(quote! {
//         #[nidrs::macros::__test]
//         #input2
//     });
// }

// #[proc_macro_attribute]
// pub fn __test(args: TokenStream, input: TokenStream) -> TokenStream {
//     let input2 = input.clone();
//     let func = parse_macro_input!(input as UFnStruct);
//     println!("__test {}", func.ident.to_string());
//     return input2;
// }

#[proc_macro_attribute]
pub fn get(args: TokenStream, input: TokenStream) -> TokenStream {
    return impl_expand::route("get", args, input);
}

#[proc_macro_attribute]
pub fn post(args: TokenStream, input: TokenStream) -> TokenStream {
    return impl_expand::route("post", args, input);
}

#[proc_macro_attribute]
pub fn put(args: TokenStream, input: TokenStream) -> TokenStream {
    return impl_expand::route("put", args, input);
}

#[proc_macro_attribute]
pub fn delete(args: TokenStream, input: TokenStream) -> TokenStream {
    return impl_expand::route("delete", args, input);
}

#[proc_macro_attribute]
pub fn any(args: TokenStream, input: TokenStream) -> TokenStream {
    return impl_expand::route("any", args, input);
}

#[proc_macro_attribute]
pub fn head(args: TokenStream, input: TokenStream) -> TokenStream {
    return impl_expand::route("head", args, input);
}

#[proc_macro_attribute]
pub fn on(args: TokenStream, input: TokenStream) -> TokenStream {
    return impl_expand::route("on", args, input);
}

#[proc_macro_attribute]
pub fn options(args: TokenStream, input: TokenStream) -> TokenStream {
    return impl_expand::route("options", args, input);
}

#[proc_macro_attribute]
pub fn patch(args: TokenStream, input: TokenStream) -> TokenStream {
    return impl_expand::route("patch", args, input);
}

#[proc_macro_attribute]
pub fn trace(args: TokenStream, input: TokenStream) -> TokenStream {
    return impl_expand::route("trace", args, input);
}

#[proc_macro_attribute]
pub fn __route_derive(args: TokenStream, input: TokenStream) -> TokenStream {
    return impl_expand::route_derive(args, input);
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

    ROUTES.lock().expect("Failed to lock ROUTES in controller macro").insert(ident.to_string(), Vec::new());

    TokenStream::from(quote! {
        #[nidrs::meta(nidrs::datasets::ServiceType::from("Controller"))]
        #[nidrs::meta(nidrs::datasets::ServiceName::from(#ident_name))]
        #[nidrs::meta(nidrs::datasets::ControllerPath::from(#path))]
        #[nidrs::macros::__service_derive(Controller)]
        #func
    })
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
        #[nidrs::macros::__service_derive(Service)]
        #func
    });
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
        #[nidrs::macros::__service_derive(Interceptor)]
        #func
    });
}

#[syn_args::derive::declare(def::Expr)]
#[syn_args::derive::proc_attribute]
pub fn __service_derive(args: Args, input: TokenStream) -> TokenStream {
    let service_type = match args {
        Args::F1(v) => match v.to_path_name().expect("Failed to get path name in __service_derive").as_str() {
            "Controller" => ServiceType::Controller,
            "Service" => ServiceType::Service,
            "Interceptor" => ServiceType::Interceptor,
            _ => panic!("Invalid service type"),
        },
    };
    impl_expand::__service_derive(service_type, input)
}

#[syn_args::derive::declare(args::ModuleOptions)]
#[syn_args::derive::proc_attribute]
pub fn __module_derive(args: Args, input: TokenStream) -> TokenStream {
    // 解析宏的参数
    let module_options: args::ModuleOptions = {
        if let Args::F1(options) = args {
            options
        } else {
            panic!("Invalid argument");
        }
    };

    let func = parse_macro_input!(input as ItemStruct);
    let (impl_generics, ty_generics, where_clause) = func.generics.split_for_impl();
    let ident = func.ident.clone();
    let ident_name = ident.to_string();

    let controller_register_tokens = impl_expand::expand_controller_register(ident_name.clone(), &module_options.controllers);
    let service_register_tokens = impl_expand::expand_service_register(ident_name.clone(), &module_options.services);

    let all_interceptors = impl_expand::merge_defaults_interceptors(module_options.interceptors.clone());
    let interceptor_register_tokens = impl_expand::expand_interceptor_register(ident_name.clone(), &all_interceptors);
    let (import_names_tokens, imports_register_tokens) = impl_expand::expand_imports_register(ident_name.clone(), &module_options.imports, &func);
    // let imports_register_names = args.imports.clone().iter().map(|import_tokens| import_tokens.to_string()).collect::<Vec<String>>();
    let exports_names_tokens = impl_expand::expand_exports_append(&module_options.exports);

    let services_dep_inject_tokens: TokenStream2 = impl_expand::expand_dep_inject("get_service", ident_name.clone(), &module_options.services);
    let controller_dep_inject_tokens = impl_expand::expand_dep_inject("get_controller", ident_name.clone(), &module_options.controllers);
    let interceptor_dep_inject_tokens = impl_expand::expand_dep_inject("get_interceptor", ident_name.clone(), &module_options.interceptors);

    let trigger_on_module_init_tokens: TokenStream2 = impl_expand::expand_events_trigger(ident_name.clone(), "on_module_init");
    let trigger_on_module_destroy_tokens = impl_expand::expand_events_trigger(ident_name.clone(), "on_module_destroy");

    let module_meta_tokens = cmeta::CMeta::build_tokens();
    let is_global_tokens = if let Some(CMetaValue::Bool(bool)) = cmeta::CMeta::get_stack_data("Global") { bool } else { false };
    println!("// module {:?}", ident.to_string());
    {
        ROUTES.lock().expect("Failed to lock ROUTES in module derive").clear();
        EVENTS.lock().expect("Failed to lock EVENTS in module derive").clear();
    }
    current_module::end_mod();

    let derives_tokens: Vec<TokenStream2> = merge_derives(&func, &["Default"]);

    return TokenStream::from(quote! {
        #(#derives_tokens)*
        #func

        impl #impl_generics nidrs::Module for #ident #ty_generics #where_clause  {
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

        impl #impl_generics nidrs::ImplMeta for #ident #ty_generics #where_clause{
            fn __meta(&self) -> nidrs::InnerMeta {
                #module_meta_tokens
            }
        }
    });
}

#[proc_macro_attribute]
pub fn module(args: TokenStream, input: TokenStream) -> TokenStream {
    let input2 = TokenStream2::from(input);
    let args2 = TokenStream2::from(args);

    TokenStream::from(quote! {
        #[nidrs::macros::meta(__ = true)]
        #[nidrs::macros::__module_derive(#args2)]
        #input2
    })
}

#[proc_macro_attribute]
pub fn on_module_init(args: TokenStream, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemFn);

    let ident = func.sig.ident.clone();
    let name = ident.to_string();

    let current_service_name: String =
        cmeta::CMeta::get_stack_data("ServiceName").expect(&format!("[on_module_init] {} ServiceName not found", name));

    EVENTS.lock().expect("Failed to lock EVENTS in on_module_init").entry("on_module_init".to_string()).or_insert(vec![]).push((current_service_name, name));

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

    EVENTS.lock().expect("Failed to lock EVENTS in on_module_destroy").entry("on_module_destroy".to_string()).or_insert(vec![]).push((current_service_name, name));

    return TokenStream::from(quote! {
        #func
    });
}

#[syn_args::derive::declare(def::Expr, def::Extends<def::Expr>)]
#[syn_args::derive::proc_attribute]
pub fn uses(args: Args, input: TokenStream) -> TokenStream {
    let args: Vec<def::Expr> = match args {
        Args::F1(first, other) => {
            let mut args = vec![first];
            args.append(&mut other.clone());
            args
        }
        _ => panic!("Invalid argument"),
    };
    let raw = TokenStream2::from(input.clone());
    let func = parse_macro_input!(input as UFnStruct);
    let used_ident = &func.ident;
    let inter_names = args.iter().map(|arg| arg.to_path_name().expect("Failed to get path name in uses macro")).collect::<Vec<String>>();

    let expand = match &func.typ {
        TokenType::Fn(item) => {
            quote! {
                #[nidrs::meta(method_uses = [#(#inter_names),*])]
            }
        }
        TokenType::Struct(item) => {
            quote! {
                #[nidrs::meta(service_uses = [#(#inter_names),*])]
            }
        }
        _ => panic!("Invalid argument"),
    };

    return quote! {
        #expand
        #raw
    }
    .into();
}

// #[syn_args::derive::declare(def::Expr, def::Extends<def::Expr>)]
// #[syn_args::derive::proc_attribute]
// pub fn default_uses(args: Args, input: TokenStream) -> TokenStream {
//     let args: Vec<def::Expr> = match args {
//         Args::F1(first, other) => {
//             let mut args = vec![first];
//             args.append(&mut other.clone());
//             args
//         }
//         _ => panic!("Invalid argument"),
//     };
//     let inter_names = args.iter().map(|arg| arg.to_path_name().unwrap()).collect::<Vec<String>>();

//     DEFAULT_INTERS.lock().unwrap().append(&mut inter_names.clone());

//     return input;
// }

#[proc_macro_attribute]
pub fn meta(args: TokenStream, input: TokenStream) -> TokenStream {
    let raw: TokenStream = input.clone();
    let fun = parse_macro_input!(input as UFnStruct);

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

    return raw;
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
