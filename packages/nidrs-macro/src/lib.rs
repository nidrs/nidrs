#![allow(warnings, unused)]
#![feature(proc_macro_span)]
extern crate proc_macro;

use std::{
    any::Any, borrow::BorrowMut, cell::RefCell, collections::HashMap, ops::Add, str::FromStr, sync::{Arc, Mutex}
};

use once_cell::sync::Lazy;
use proc_macro::{Ident, Span, TokenStream};
use proc_macro2::{Punct, TokenTree};
use quote::{quote, ToTokens};
use syn::{meta, parse::{Parse, ParseStream}, parse_str, Expr, ExprArray, ExprCall, PatPath, Stmt, Token};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, spanned::Spanned, FnArg, ItemFn, ItemStruct, PatType, Type};
use proc_macro2::TokenStream as TokenStream2;

mod args_parse;
use args_parse::*;

static CURRENT_CONTROLLER: Mutex<Option<ControllerMeta>> = Mutex::new(None);
static ROUTES: Lazy<Mutex<HashMap<String, HashMap<String, RouteMeta>>>> = Lazy::new(||Mutex::new(HashMap::new())); // HashMap<ControllerName, HashMap<RouteName, RouteMeta>>
static CURRENT_SERVICE: Mutex<Option<ServiceMeta>> = Mutex::new(None);
static EVENTS: Lazy<Mutex<HashMap<String, Vec<(String, String)>>>> = Lazy::new(||Mutex::new(HashMap::new())); // HashMap<EventName, Vec<(ServiceName,FName)>>
static INTERS: Lazy<Mutex<HashMap<String, Vec<String>>>> = Lazy::new(||Mutex::new(HashMap::new())); // HashMap<ServiceName, Vec<InterName>>

static MERGE_MACRO: Lazy<Mutex<Vec<String>>> = Lazy::new(||Mutex::new(vec![]));

#[derive(Debug, Clone)]
struct RouteMeta {
    method: String,
    path: String,
    name: String,
    func_args: Vec<String>,
    is_body: bool,
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
    
    let func = parse_macro_input!(input as ItemStruct);
    
    let ident = func.ident.clone();
    
    // println!("controller {} {:?}", ident.to_string(), func.attrs);
    CURRENT_CONTROLLER.lock().unwrap().replace(ControllerMeta {
        name: ident.to_string(),
        path: path,
    });
    ROUTES.lock().unwrap().insert(ident.to_string(), HashMap::new());

    let inject_tokens = gen_service_inject_tokens("ControllerService", &func);

    TokenStream::from(quote! {
        #func

        #inject_tokens
    })
}

#[proc_macro_attribute]
pub fn module(args: TokenStream, input: TokenStream) -> TokenStream {
    // 解析宏的参数
    let args = parse_macro_input!(args as ModuleArgs);
    let func = parse_macro_input!(input as ItemStruct);
    let ident = func.ident.clone();

    let controller_register_tokens= gen_controller_register_tokens(args.controllers.clone());
    let service_register_tokens= gen_service_register_tokens(args.services.clone());
    let interceptor_register_tokens= gen_interceptor_register_tokens(args.interceptors.clone());
    let imports_register_tokens = gen_imports_register_tokens(args.imports.clone());

    let services_dep_inject_tokens = gen_dep_inject_tokens("services", args.services.clone());
    let controller_dep_inject_tokens = gen_dep_inject_tokens("controllers", args.controllers.clone());
    let interceptor_dep_inject_tokens = gen_dep_inject_tokens("interceptors", args.interceptors.clone());

    let events_trigger_tokens =  gen_events_trigger_tokens();
    
    // println!("module {:?}", ident.to_string());
    CURRENT_CONTROLLER.lock().unwrap().replace(ControllerMeta {
        name: "".to_string(),
        path: "".to_string(),
    });
    ROUTES.lock().unwrap().clear();
    EVENTS.lock().unwrap().clear();
    INTERS.lock().unwrap().clear();
    MERGE_MACRO.lock().unwrap().clear();

    return TokenStream::from(quote! {
        #func
        
        impl nidrs::Module for #ident {
            fn init(self, mut ctx: nidrs::ModuleCtx) -> nidrs::ModuleCtx{
                use nidrs::{Service, ControllerService, InterceptorService, InterCtx, Interceptor, ModuleCtx, StateCtx, ImplMeta};
                if ctx.modules.contains_key(stringify!(#ident)) {
                    return ctx;
                }
                ctx.modules.insert(stringify!(#ident).to_string(), Box::new(self) as Box<dyn std::any::Any>);
                nidrs_macro::log!("Registering module {}.", stringify!(#ident));
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
                    #events_trigger_tokens
                // }

                ctx
            }
        }
    });
}


#[proc_macro_attribute]
pub fn injectable(args: TokenStream, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemStruct);
    CURRENT_SERVICE.lock().unwrap().replace(ServiceMeta {
        name: func.ident.to_string(),
    });

    let inject_tokens = gen_service_inject_tokens("Service", &func);

    return TokenStream::from(quote! {
        #func

        #inject_tokens
    });
}

#[proc_macro_attribute]
pub fn interceptor(args: TokenStream, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemStruct);
    CURRENT_SERVICE.lock().unwrap().replace(ServiceMeta {
        name: func.ident.to_string(),
    });

    let inject_tokens = gen_service_inject_tokens("InterceptorService", &func);

    return TokenStream::from(quote! {
        #func

        #inject_tokens
    });
}

#[proc_macro_attribute]
pub fn on_module_init(args: TokenStream, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemFn);
    
    let ident = func.sig.ident.clone();
    let current_service = CURRENT_SERVICE.lock().unwrap().clone();

    EVENTS.lock().unwrap()
        .entry("on_module_init".to_string())
        .or_insert(vec![])
        .push((current_service.unwrap().name, ident.to_string()));

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
    let inter_names = args.items.iter().map(|arg| {
        if let Expr::Path(path) = arg {
            path.to_token_stream().to_string()
        } else {
            panic!("Invalid argument");
        }
    }).collect::<Vec<String>>();
    if let TokenType::Fn(_) = input_type.typ {
        let controller_name = CURRENT_CONTROLLER.lock().unwrap().as_ref().unwrap().name.clone();
        let hook_name = controller_name + ":" + &used_ident.to_string();
        INTERS.lock().unwrap()
            .entry(hook_name)
            .or_insert(vec![])
            .append(&mut inter_names.clone());
    } else if let TokenType::Struct(_) = input_type.typ {
        INTERS.lock().unwrap()
            .entry(used_ident.to_string())
            .or_insert(vec![])
            .append(&mut inter_names.clone());
    }
    return input;
}

#[proc_macro_attribute]
pub fn meta(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as MetaArgs);
    let raw_input = TokenStream2::from(input.clone());
    let func = parse_macro_input!(input as InterceptorArgs);
    let func_ident = func.ident.clone();
    let func_name = func.ident.to_string();
    let meta_tokens = args.kv.iter().map(|(key, value)| {
        // value parse expr
        let exp = parse_str::<Expr>(&value).unwrap();
        // println!("// meta {} {} {:?}", key, value, exp);

        let v = match exp {
            Expr::Array(arr) => {
                // arr to vec
                let arr = arr.elems.iter().map(|elem| {
                    elem.to_owned()
                }).collect::<Vec<Expr>>();
                quote! {
                    Vec::from([#(#arr),*])
                }
            }
            _ => {exp.to_token_stream()}
        };



        quote! {
            meta.set(#key.to_string(), #v);
        }
    }).collect::<Vec<TokenStream2>>();
    let meta_tokens = TokenStream2::from(quote! {
        #(#meta_tokens)*
    });
    MERGE_MACRO.lock().unwrap().push(meta_tokens.to_string());

    // if let TokenType::Struct(_) = func.typ {

    // } else if  let TokenType::Fn(p) = func.typ {
    //     println!("meta {} {:?}", func_name, p.to_token_stream().to_string());
    // } 

    return TokenStream::from(quote! {
        #raw_input
    });
}

fn is_macro(s: ItemStruct, name: &str)->bool{
    !s.attrs.iter().any(|attr: &syn::Attribute| {
        if let syn::Meta::List(name_value) = attr.meta.to_owned() {
            if name_value.path.is_ident(name) {
                return true;
            }
        }
        return false;
    })
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

/// auto add a meta #[meta(version = String)]
#[proc_macro_attribute]
pub fn version(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as ExprList);
    let raw_input = TokenStream2::from(input.clone());

    let version = args.items.iter().map(|arg| {
        if let Expr::Lit(lit) = arg {
            if let syn::Lit::Str(str) = lit.to_owned().lit {
                str.value().trim().to_string()
            } else {
                panic!("Invalid argument")
            }
        } else {
            panic!("Invalid argument")
        }
    }).collect::<Vec<String>>().first().unwrap().clone();

    return TokenStream::from(quote! {
        #[meta(version = #version)]
        #raw_input
    });
}

#[proc_macro_attribute]
pub fn main(args:TokenStream, input: TokenStream) -> TokenStream {

    let func = parse_macro_input!(input as ItemFn);
    let ident = func.sig.ident.clone();

    let main_tokens = TokenStream2::from(quote! {
        #func        
    });


    return main_tokens.into();
}

fn route(method:&str, args: TokenStream, input: TokenStream)-> TokenStream{
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

    let vis = func.vis.clone();
    let ident = func.sig.ident.clone();
    let mut pindex = 0;
    let mut is_body = false;
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
                }
                let pat = format!("p{}", pindex);
                let pat_indent = syn::Ident::new(&pat, Span::call_site().into());
                pindex += 1;
                quote! {
                    #pat_indent
                }
            }
            _ => quote! {},
        }).map(|arg| arg.to_string()).filter(|v|!v.is_empty()).collect::<Vec<String>>();

    let route = RouteMeta {
        method: method.to_string(),
        path: path,
        name: name.clone(),
        func_args,
        is_body,
    };

    let mut binding = ROUTES.lock().unwrap();
    let controller = binding.get_mut(&CURRENT_CONTROLLER.lock().unwrap().as_ref().unwrap().name).unwrap();
    controller.insert(name.clone(), route);

    let prev_meta_tokens = gen_meta_tokens();

    let meta_ident = syn::Ident::new(format!("__meta_{}", name).as_str(), Span::call_site().into());
    TokenStream::from(quote! {
        #func

        pub fn #meta_ident(&self)->nidrs::Meta{
            let mut meta = nidrs::Meta::new();
            #prev_meta_tokens
            meta
        }
    })
}

fn gen_controller_register_tokens(services: Vec<TokenStream2>) -> TokenStream2 {
    let binding = CURRENT_CONTROLLER.lock().unwrap();
    let current_controller = binding.as_ref().unwrap();
    let controller_path = current_controller.path.clone();
    let controller_tokens= services.iter().map(|controller_token| {
        let controller_str = controller_token.to_string();
        let binding = ROUTES.lock().unwrap();
        let controller = binding.get(&controller_str).unwrap();
        let controller_ident = syn::Ident::new(&controller_str, Span::call_site().into());
        let router_path = controller.iter().map(|(name, route)| {
            let method = route.method.clone();
            let method_ident = syn::Ident::new(&method, Span::call_site().into());
            let path = controller_path.clone() + &route.path.clone();
            let route_name = syn::Ident::new(&route.name, Span::call_site().into());
            // println!("route {} {:?}", route_name.to_string(), route.func_args);
            let func_args = str_args_to_indent(route.func_args.clone());
            let noop_ids = vec![];
            let inter_name = controller_str.clone() + ":" + &route_name.to_string();
            let binding = INTERS.lock().unwrap();
            let struct_inter_ids = binding.get(&controller_str).unwrap_or(&noop_ids);
            let inter_ids = binding.get(&inter_name).unwrap_or(&noop_ids);
            let inter_ids = struct_inter_ids.iter().chain(inter_ids.iter()).collect::<Vec<&String>>();

            // interceptor handle
            let inter_ids = inter_ids.iter().map(|inter_id| {
                syn::Ident::new(inter_id, Span::call_site().into())
            }).collect::<Vec<syn::Ident>>();
            let mut i = 0;
            let inter_tokens = inter_ids.iter().map(|inter_id| {
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
                        quote!{
                            let t_body = ctx.body;
                        }
                    } else {
                        quote!{}
                    };
                    let func_args = str_args_to_indent(t_vec);
                    tokens.push(quote!{
                        let #prev_t_inter_fn_indent = |ctx: InterCtx<_>| async move {
                            #body_tokens
                            t_controller.#route_name(#func_args).await
                        };
                    });
                }
                if i == inter_ids.len() {
                    tokens.push(quote!{
                        #prev_t_interceptor_ident.interceptor(ctx, #prev_t_inter_fn_indent).await
                    });
                } else {
                    tokens.push(quote!{
                        let #t_inter_fn_indent = |ctx: InterCtx<_>| async move {
                            #prev_t_interceptor_ident.interceptor(ctx, #prev_t_inter_fn_indent).await
                        };
                    });
                }
                
                (
                    quote!{
                        let #prev_t_interceptor_ident = ctx.interceptors.get(stringify!(#inter_id)).unwrap();
                        let #prev_t_interceptor_ident = #prev_t_interceptor_ident.downcast_ref::<std::sync::Arc<#inter_id>>().unwrap();
                        let #prev_t_interceptor_ident = #prev_t_interceptor_ident.clone();
                    },
                    quote!{
                        #(#tokens)*
                    },
                    quote!{
                    },
                    quote!{
                    },
                )
            }).collect::<Vec<(TokenStream2, TokenStream2, TokenStream2, TokenStream2)>>();
            let def_inter_tokens = inter_tokens.iter().map(|(tokens, _,  _, _)| {
                tokens.clone()
            }).collect::<Vec<TokenStream2>>();
            let def_inter_tokens = TokenStream2::from(quote! {
                #(#def_inter_tokens)*
            });
            let def_clone_inter_tokens = inter_tokens.iter().map(|(_, tokens,  _, _)| {
                tokens.clone()
            }).collect::<Vec<TokenStream2>>();
            let ctx_body_tokens = if route.is_body {
                let last_arg_indent =  syn::Ident::new(route.func_args.last().unwrap().as_str(), Span::call_site().into());
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
                let mut t_meta = nidrs::Meta::new();
                t_meta.extend(meta);
                let ctx = InterCtx {
                    meta: t_meta,
                    parts,
                    body: t_body,
                };
                #(#def_clone_inter_tokens)*
            });

            // meta handle
            let struct_meta_tokens = quote! {
                let mut meta = nidrs::get_meta(t_controller.clone());
            };
            let method_meta = syn::Ident::new(format!("__meta_{}", route_name).as_str(), Span::call_site().into());
            let method_meta_tokens = quote! {
                    let t_meta = t_controller.#method_meta();
                    meta.merge(t_meta);
                };
            let meta_tokens = TokenStream2::from(quote! {
                #struct_meta_tokens
                #method_meta_tokens
                let meta = std::sync::Arc::new(meta);
            });
            let handler = if inter_ids.is_empty() {
                quote!{
                    |#func_args| async move {
                        t_controller.#route_name(#func_args).await
                    }
                }
            } else {
                quote!{
                    |parts, #func_args| async move {
                        #def_clone_inter_tokens
                    }
                }
            };
            quote! {
                let t_controller = ctx.controllers.get(#controller_str).unwrap();
                let t_controller = t_controller.downcast_ref::<std::sync::Arc<controller::#controller_ident>>().unwrap();
                let t_controller = t_controller.clone();

                #def_inter_tokens

                #meta_tokens

                let version = *meta.get::<&str>("version").unwrap_or(&ctx.defaults.default_version);
                let disable_default_prefix = *meta.get::<bool>("disable_default_prefix").unwrap_or(&false);
                let path = if disable_default_prefix { #path.to_string() } else { nidrs::template_format(&format!("{}{}", ctx.defaults.default_prefix, #path), [("version", version)]) };
                nidrs_macro::log!("Registering router '{} {}'.", #method.to_uppercase(), path);
                let router = axum::Router::new().route(
                    &path,
                    axum::routing::#method_ident(#handler),
                );
                ctx.routers.push(router);
            }
        }).collect::<Vec<TokenStream2>>();
        let router_path = TokenStream2::from(quote! {

            nidrs_macro::log!("Registering controller {}.", #controller_str);

            #(#router_path)*
        });
        
        quote! {
            ctx.controllers.insert(#controller_str.to_string(), Box::new(std::sync::Arc::new(controller::#controller_ident::default())));
            
            #router_path
        }
    }).collect::<Vec<TokenStream2>>();
    let controller_tokens = TokenStream2::from(quote! {
        #(#controller_tokens)*
    });
    return controller_tokens;
}

fn gen_service_register_tokens(services: Vec<TokenStream2>) -> TokenStream2 {
    let controller_tokens= services.iter().map(|controller_tokens| {
        let controller_str = controller_tokens.to_string();
        let controller_ident = controller_tokens;
        
        quote! {
            nidrs_macro::log!("Registering service {}.", #controller_str);
            ctx.services.insert(#controller_str.to_string(), Box::new(std::sync::Arc::new(#controller_ident::default())) as Box<dyn std::any::Any>);
        }
    }).collect::<Vec<TokenStream2>>();
    let controller_tokens = TokenStream2::from(quote! {
        #(#controller_tokens)*
    });
    return controller_tokens;
}

fn gen_interceptor_register_tokens(services: Vec<TokenStream2>) -> TokenStream2 {
    let controller_tokens= services.iter().map(|controller_tokens| {
        let controller_str = controller_tokens.to_string();
        let controller_ident = controller_tokens;
        
        quote! {
            nidrs_macro::log!("Registering interceptor {}.", #controller_str);
            ctx.interceptors.insert(#controller_str.to_string(), Box::new(std::sync::Arc::new(#controller_ident::default())) as Box<dyn std::any::Any>);
        }
    }).collect::<Vec<TokenStream2>>();
    let controller_tokens = TokenStream2::from(quote! {
        #(#controller_tokens)*
    });
    return controller_tokens;
}

fn gen_imports_register_tokens(imports: Vec<TokenStream2>) -> TokenStream2 {
    let imports = imports.iter().map(|import_tokens| {
        let import = import_tokens.to_string();

        if import.contains("for_root") {
            let import_call = syn::parse2::<ExprCall>(import_tokens.clone()).unwrap();
            if let Expr::Path(path)  = import_call.func.as_ref(){
                let module_ident = path.path.segments.first().unwrap().ident.clone();
                quote! {
                    let dyn_module = #import_call;
                    let mut dyn_module_services = dyn_module.services;
                    dyn_module_services.drain().for_each(|(k, v)| {
                        nidrs_macro::log!("Registering dyn service {}.", k);
                        ctx.services.insert(k.to_string(), v);
                    });
                    let ctx = #module_ident::default().init(ctx);
                }
            }else {
                panic!("Invalid import.")
            }
        } else {
            quote! {
                let ctx = #import_tokens::default().init(ctx);
            }
        }
    }).collect::<Vec<TokenStream2>>();

    let imports = TokenStream2::from(quote! {
        #(#imports)*
    });

    return imports;
}

fn gen_dep_inject_tokens(con: &str, services: Vec<TokenStream2>) -> TokenStream2 {
    let con_ident = syn::Ident::new(con, Span::call_site().into());
    let controller_tokens= services.iter().map(|tokens| {
        let controller_str = tokens.to_string();
        let controller_ident = tokens;
        
        quote! {
            let t = ctx.#con_ident.get(#controller_str).unwrap();
            let t = t.downcast_ref::<std::sync::Arc<#controller_ident>>().unwrap();
            let t = t.clone();
            nidrs_macro::log!("Injecting {}.", #controller_str);
            let ctx = t.inject(ctx);
        }
    }).collect::<Vec<TokenStream2>>();
    let controller_tokens = TokenStream2::from(quote! {
        #(#controller_tokens)*
    });
    return controller_tokens;
}

fn gen_service_inject_tokens(service_type: &str, func: &ItemStruct) -> TokenStream2{
    let is_service = "Service".contains(service_type);
    let service_type = syn::Ident::new(service_type, Span::call_site().into());
    let ident = func.ident.clone();
    let ident_str = ident.to_string();

    let fields = if let syn::Fields::Named(fields) = &func.fields {
        fields.named.iter().map(|field| {
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
                            let injected_type_str = injected_type.to_string();
                            quote! {
                                let service = ctx.services.get(#injected_type_str).expect(format!("[{}] Service {} not register.", #ident_str, #injected_type_str).as_str());
                                let service = service.downcast_ref::<std::sync::Arc<#injected_type>>().unwrap();
                                self.#field_ident.inject(service.clone());
                            }
                        } else{
                            quote! {}
                        }
                    }else {
                        quote! {}
                    }
                } else {
                    quote! {}
                }
            }else{
                quote! {}
            }
        }).collect::<Vec<TokenStream2>>()
    } else {
        vec![]
    };
    let middle_tokens = if is_service { quote! {} } else { quote! {
        impl nidrs::#service_type for #ident {}
    } };

    let prev_meta_tokens = gen_meta_tokens();

    let inject_tokens = TokenStream2::from(quote! {
        #middle_tokens
        impl nidrs::Service for #ident {
            fn inject(&self, ctx: nidrs::ModuleCtx) -> nidrs::ModuleCtx{
                #(#fields)*
                ctx
            }
        }

        impl nidrs::ImplMeta for #ident{
            fn __meta() -> nidrs::Meta {
                let mut meta = nidrs::Meta::new();
                #prev_meta_tokens
                meta.set("service_name".to_string(), stringify!(#ident));
                meta.set("service_type".to_string(), stringify!(#service_type));
                meta
            }
        }
    });

    return inject_tokens;

}

fn gen_meta_tokens() -> TokenStream2 {
    let prev_meta_tokens = MERGE_MACRO.lock().unwrap().drain(..).map(
        |tokens| {
            let tokens = TokenStream2::from_str(tokens.as_str()).unwrap();
            quote! {
                #tokens
            }
        }
    ).collect::<Vec<TokenStream2>>();
    let prev_meta_tokens: TokenStream2 = TokenStream2::from(quote! {
        #(#prev_meta_tokens)*
    });
    prev_meta_tokens
}

fn gen_events_trigger_tokens() -> TokenStream2 {
    let binding = EVENTS.lock().unwrap();
    let on_module_init = binding.get("on_module_init");
    if let None = on_module_init {
        return TokenStream2::new();
    }
    let events_trigger_tokens = on_module_init.unwrap().iter().map(|(service, func)| {
        let service_ident = syn::Ident::new(service, Span::call_site().into());
        let func_ident = syn::Ident::new(func, Span::call_site().into());
        quote! {
            let service = ctx.services.get(#service).unwrap();
            let service = service.downcast_ref::<std::sync::Arc<#service_ident>>().unwrap();
            let service = service.clone();
            nidrs_macro::log!("Triggering event on_module_init for {}.", #service);
            service.#func_ident();
        }
    }).collect::<Vec<TokenStream2>>();
    let events_trigger_tokens = TokenStream2::from(quote! {
        #(#events_trigger_tokens)*
    });
    return events_trigger_tokens;
}

fn str_args_to_indent(args: Vec<String>) -> TokenStream2 {
    let args = args.iter().map(|arg| {
        let arg_ident = syn::Ident::new(arg, Span::call_site().into());
        quote! {
            #arg_ident
        }
    }).collect::<Vec<TokenStream2>>();
    let args = TokenStream2::from(quote! {
        #(#args),*
    });
    return args;
}