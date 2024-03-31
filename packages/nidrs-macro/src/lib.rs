#![allow(warnings, unused)]
extern crate proc_macro;

use std::{
    any::Any, borrow::BorrowMut, collections::HashMap, sync::{Arc, Mutex}
};

use once_cell::sync::Lazy;
use proc_macro::{Ident, Span, TokenStream};
use proc_macro2::Punct;
use quote::{quote, ToTokens};
use syn::{parse::{Parse, ParseStream}, Expr, Token};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, spanned::Spanned, FnArg, ItemFn, ItemStruct, PatType, Type};
use proc_macro2::TokenStream as TokenStream2;

static CURRENT_CONTROLLER: Mutex<Option<ControllerMeta>> = Mutex::new(None);
static ROUTES: Lazy<Mutex<HashMap<String, HashMap<String, RouteMeta>>>> = Lazy::new(||Mutex::new(HashMap::new())); // HashMap<ControllerName, HashMap<RouteName, RouteMeta>>
static CURRENT_SERVICE: Mutex<Option<ServiceMeta>> = Mutex::new(None);
static EVENTS: Lazy<Mutex<HashMap<String, Vec<(String, String)>>>> = Lazy::new(||Mutex::new(HashMap::new())); // HashMap<EventName, Vec<(ServiceName,FName)>>

#[derive(Debug, Clone)]
struct RouteMeta {
    method: String,
    path: String,
    name: String,
    handler: String,
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

#[derive(Debug, Clone)]
struct ModuleArgs {
    imports: Vec<String>,
    controllers: Vec<String>,
    services: Vec<String>,
    exports: Vec<String>,
}

impl Parse for ModuleArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let args:syn::Block = input.parse()?;
        // let args = parse_macro_input!(input.parse::<Expr>()) as Expr;

        let mut imports = Vec::new();
        let mut controllers = Vec::new();
        let mut services = Vec::new();
        let mut exports = Vec::new();

        let parse_args_map = args.stmts.iter().map(|stmt| {
            if let syn::Stmt::Expr(exp, _) = stmt {
                if let syn::Expr::Assign(assign) = exp {
                    if let syn::Expr::Path(path) = *assign.left.clone() {
                        return (path.path.segments.first().unwrap().ident.to_string(), if let syn::Expr::Array(array) = *assign.right.clone() {
                            array.elems.iter().map(|elem| {
                                if let syn::Expr::Path(path) = elem {
                                    return path.path.segments.first().unwrap().ident.to_string();
                                }
                                return "".to_string();
                            }).collect::<Vec<String>>()
                        } else {
                            vec![]
                        });
                    }
                }
    
            }
            panic!("Invalid argument");
        }).collect::<HashMap<String, Vec<String>>>();

        parse_args_map.iter().for_each(|(k, v)| {
            match k.as_str() {
                "imports" => imports = v.clone(),
                "controllers" => controllers = v.clone(),
                "services" => services = v.clone(),
                "exports" => exports = v.clone(),
                _ => {}
            }
        });
    
        Ok(ModuleArgs {
            imports,
            controllers,
            services,
            exports,
        })
    }
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

    CURRENT_CONTROLLER.lock().unwrap().replace(ControllerMeta {
        name: ident.to_string(),
        path: path,
    });
    ROUTES.lock().unwrap().insert(ident.to_string(), HashMap::new());

    let inject_tokens = service_inject_tokens("Controller", &func);

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

    let controller_tokens= controller_register_tokens(args.controllers.clone());
    let service_tokens= service_register_tokens(args.services.clone());
    let import_tokens = imports_register_tokens(args.imports.clone());

    let services_dep_inject_tokens = dep_inject_tokens("services", args.services.clone());
    let controller_dep_inject_tokens = dep_inject_tokens("controllers", args.controllers.clone());

    let events_trigger_tokens =  gen_events_trigger_tokens();
    
    // println!("event {} {:?}", ident, EVENTS.lock().unwrap());

    CURRENT_CONTROLLER.lock().unwrap().replace(ControllerMeta {
        name: "".to_string(),
        path: "".to_string(),
    });
    ROUTES.lock().unwrap().clear();
    EVENTS.lock().unwrap().clear();

    // 返回原始的输入，因为我们并没有修改它
    return TokenStream::from(quote! {
        #func
        
        impl nidrs::Module for #ident {
            fn register(self, ctx: &nidrs::ModuleCtx) -> nidrs::DynamicModule {
                use nidrs::Service;
                use nidrs::Controller;
                if ctx.modules.lock().unwrap().contains_key(stringify!(#ident)) {
                    return nidrs::DynamicModule{};
                }
                ctx.modules.lock().unwrap().insert(stringify!(#ident).to_string(), Box::new(self) as Box<dyn std::any::Any>);
                println!("Registering module {}.", stringify!(#ident));
                {
                    #controller_tokens
    
                    #service_tokens
                }
                {
                    #import_tokens
                }
                {
                    let services = ctx.services.lock().unwrap();
                    let controllers = ctx.controllers.lock().unwrap();

                    #services_dep_inject_tokens
    
                    #controller_dep_inject_tokens
                }

                {
                    let services = ctx.services.lock().unwrap();

                    #events_trigger_tokens
                }

                nidrs::DynamicModule{}
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

    let inject_tokens = service_inject_tokens("Service", &func);

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


#[proc_macro]
pub fn get_route_meta(input: TokenStream) -> TokenStream {
    return input;
}



fn controller_register_tokens(services: Vec<String>) -> TokenStream2 {
    let binding = CURRENT_CONTROLLER.lock().unwrap();
    let current_controller = binding.as_ref().unwrap();
    let controller_path = current_controller.path.clone();
    let controller_tokens= services.iter().map(|controller_str| {
        let binding = ROUTES.lock().unwrap();
        let controller = binding.get(controller_str).unwrap();
        let controller_ident = syn::Ident::new(controller_str, Span::call_site().into());
        let router_path = controller.iter().map(|(name, route)| {
            let method = route.method.clone();
            let method_ident = syn::Ident::new(&method, Span::call_site().into());
            let path = controller_path.clone() + &route.path.clone();
            let handler = route.handler.clone();
            let handler = syn::parse_str::<Expr>(&handler).unwrap();
            quote! {
                let t_controller = controllers.get(#controller_str).unwrap();
                let t_controller = t_controller.downcast_ref::<std::sync::Arc<controller::#controller_ident>>().unwrap();
                let t_controller = t_controller.clone();
                println!("Registering router '{} {}'.", #method.to_uppercase(),#path);
                ctx.routers.lock().unwrap().push(axum::Router::new().route(
                    #path,
                    axum::routing::#method_ident(#handler),
                ));
            }
        }).collect::<Vec<TokenStream2>>();
        let router_path = TokenStream2::from(quote! {
            #(#router_path)*
        });
        
        quote! {
            ctx.controllers.lock().unwrap().insert(#controller_str.to_string(), Box::new(std::sync::Arc::new(controller::#controller_ident::default())));
            let controllers = ctx.controllers.lock().unwrap();
            
            #router_path
        }
    }).collect::<Vec<TokenStream2>>();
    let controller_tokens = TokenStream2::from(quote! {
        #(#controller_tokens)*
    });
    return controller_tokens;
}


fn service_register_tokens(services: Vec<String>) -> TokenStream2 {
    let controller_tokens= services.iter().map(|controller_str| {
        let controller_ident = syn::Ident::new(controller_str, Span::call_site().into());
        
        quote! {
            println!("Registering service {}.", #controller_str);
            ctx.services.lock().unwrap().insert(#controller_str.to_string(), Box::new(std::sync::Arc::new(service::#controller_ident::default())) as Box<dyn std::any::Any>);
        }
    }).collect::<Vec<TokenStream2>>();
    let controller_tokens = TokenStream2::from(quote! {
        #(#controller_tokens)*

        let services = ctx.services.lock().unwrap();
    });
    return controller_tokens;
}

fn imports_register_tokens(imports: Vec<String>) -> TokenStream2 {
    let imports = imports.iter().map(|import| {
        let import_let = (import.to_string() + "_module").to_string();
        let import_let_indent = syn::Ident::new(&import_let, Span::call_site().into());
        let import_ident = syn::Ident::new(import, Span::call_site().into());
        
        quote! {
            let #import_let_indent = #import_ident::default();
            #import_let_indent.register(ctx);
        }
    }).collect::<Vec<TokenStream2>>();

    let imports = TokenStream2::from(quote! {
        #(#imports)*
    });

    return imports;
}

fn dep_inject_tokens(con: &str, services: Vec<String>) -> TokenStream2 {
    let con_ident = syn::Ident::new(con, Span::call_site().into());
    let controller_tokens= services.iter().map(|controller_str| {
        let controller_ident = syn::Ident::new(controller_str, Span::call_site().into());
        
        quote! {
            let t = #con_ident.get(#controller_str).unwrap();
            let t = t.downcast_ref::<std::sync::Arc<#controller_ident>>().unwrap();
            let t = t.clone();
            println!("Injecting {}.", #controller_str);
            t.inject(&services);
        }
    }).collect::<Vec<TokenStream2>>();
    let controller_tokens = TokenStream2::from(quote! {
        #(#controller_tokens)*
    });
    return controller_tokens;
}

fn service_inject_tokens(service_type: &str, func: &ItemStruct) -> TokenStream2{
    let service_type = syn::Ident::new(service_type, Span::call_site().into());
    let ident = func.ident.clone();

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
                                let service = services.get(#injected_type_str).unwrap();
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

    let inject_tokens = TokenStream2::from(quote! {
        impl nidrs::#service_type for #ident {
            fn inject(&self, services: &std::sync::MutexGuard<std::collections::HashMap<String, Box<dyn std::any::Any>>>) {
                #(#fields)*
            }
        }
    });

    return inject_tokens;

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
    let func_args = func
        .sig
        .inputs
        .iter()
        .map(|arg| match arg {
            FnArg::Typed(PatType { pat, ty, .. }) => {
                // let pat = pat.to_token_stream();
                // let ty = ty.to_token_stream();
                let pat = format!("p{}", pindex);
                let pat_indent = syn::Ident::new(&pat, Span::call_site().into());
                pindex += 1;
                quote! {
                    #pat_indent
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

    let handler = TokenStream::from(quote! {
        |#func_args| async move {
            t_controller.#ident(#func_args).await
        }
    }).to_string();

    let route = RouteMeta {
        method: method.to_string(),
        path: path,
        name: name.clone(),
        handler
    };

    let mut binding = ROUTES.lock().unwrap();
    let controller = binding.get_mut(&CURRENT_CONTROLLER.lock().unwrap().as_ref().unwrap().name).unwrap();
    controller.insert(name.clone(), route);

    TokenStream::from(quote! {
        #func
    })
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
            let service = services.get(#service).unwrap();
            let service = service.downcast_ref::<std::sync::Arc<service::#service_ident>>().unwrap();
            let service = service.clone();
            println!("Triggering event on_module_init for {}.", #service);
            service.#func_ident();
        }
    }).collect::<Vec<TokenStream2>>();
    let events_trigger_tokens = TokenStream2::from(quote! {
        #(#events_trigger_tokens)*
    });
    return events_trigger_tokens;
}