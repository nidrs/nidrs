use std::collections::HashSet;

use super::DEFAULT_INTERS;
use super::EVENTS;
use super::ROUTES;

use nidrs_extern::helper::merge_derives;
use quote::quote;
use quote::ToTokens;
use syn::parse_macro_input;
use syn::spanned::Spanned;
use syn::Expr;
use syn::ItemFn;
use syn::Path;
use syn::Type;

use syn::ItemStruct;

use nidrs_extern::datasets::ServiceType;

use syn::ExprCall;
use syn_args::def;
use syn_args::SynArgs;

use crate::args;
use crate::cmeta;
use crate::import_path;
use crate::utils::merge_uses;

use proc_macro::Span;

use syn::PatType;

use syn::FnArg;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

pub(crate) fn route(method: &str, args: TokenStream, input: TokenStream) -> TokenStream {
    let args: args::RouteArgs = parse_macro_input!(args as SynArgs).arguments().unwrap();
    let path = match args {
        args::RouteArgs::F1(def::Option(Some(v))) => v.to_string(),
        _ => "".to_string(),
    };

    let func = parse_macro_input!(input as ItemFn);

    let route_ident = func.sig.ident.clone();
    let route_name = route_ident.to_string();

    let current_controller_name: String =
        cmeta::CMeta::get_stack_data("ServiceName").expect(&format!("[route] {} ServiceName not found", route_name));

    let mut routes = ROUTES.lock().expect("Failed to lock ROUTES in route macro");
    let controller = routes.get_mut(&current_controller_name).expect("Failed to get controller in route macro");
    controller.push(route_name.clone());

    TokenStream::from(quote! {
        #[nidrs::meta(nidrs::datasets::RouterName::from(#route_name))]
        #[nidrs::meta(nidrs::datasets::RouterMethod::from(#method))]
        #[nidrs::meta(nidrs::datasets::RouterPath::from(#path))]
        #[nidrs::__route_derive]
        #func
    })
}

pub(crate) fn route_derive(args: TokenStream, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemFn);
    let fn_ident = func.sig.ident.clone();
    let meta_fn_ident = syn::Ident::new(format!("__meta_{}", func.sig.ident.to_string()).as_str(), func.span().clone());

    println!("// route_derive {:?}", func.sig.ident.to_string());
    // println!("route_derive {:?}", func.sig.output);

    // let disable_auto_json: bool = cmeta::CMeta::get_stack("disable_auto_json").unwrap_or(cmeta::CMetaValue::Bool(false)).into();
    // let mut is_tuple = false; // AppResult<(T,T)>
    // if let syn::ReturnType::Type(_, ty) = &func.sig.output {
    //     if let syn::Type::Path(p) = ty.as_ref() {
    //         if let Some(segment) = p.path.segments.first() {
    //             // println!("route_derive {:#?}", segment);
    //             if segment.ident.to_string() == "AppResult" {
    //                 if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
    //                     if let syn::GenericArgument::Type(ty) = args.args.first().expect("Failed to get first argument in route_derive") {
    //                         if let syn::Type::Tuple(_) = ty {
    //                             is_tuple = true;
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
    // println!("// route_derive is_tuple {:?}", is_tuple);

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
                    let inter = ctx.get_interceptor::<#inter_ident>(module, #inter);
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

    // let handler_tokens = if is_tuple || disable_auto_json {
    //     quote! {
    //         r
    //     }
    // } else {
    //     quote! {
    //         match r {
    //             Ok(r) => Json(r).into_response(),
    //             Err(e) => e.into_response(),
    //         }
    //     }
    // };
    let handler_tokens = quote! {
        r
    };

    TokenStream::from(quote! {
        #func

        pub fn #meta_fn_ident(&self)->nidrs::InnerMeta{
            #meta_tokens
        }

        pub fn #route_fn_ident(&self, mut ctx: nidrs::ModuleCtx, module: &str)->nidrs::ModuleCtx{
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
            let controller_name = meta.get_data::<nidrs::datasets::ServiceName>().unwrap().value();

            let t_controller = ctx.get_controller::<Self>(module, controller_name);

            let router = nidrs::externs::axum::Router::new()
                .route(
                    &full_path,
                    nidrs::externs::axum::routing::#route_method (|#axum_args| async move {
                        #meta_token
                        let r = t_controller.#fn_ident(#func_args).await;
                        #handler_tokens
                    }),
                )
                .route_layer(nidrs::externs::axum::Extension(meta.clone()))
                #(#interceptor_uses_expand)*
                ;
            ctx.routers
                .push(nidrs::MetaRouter::new(router, meta));

            ctx
        }

    })
}

pub(crate) fn expand_controller_register(module_name: String, services: &def::Array<def::Expr>) -> TokenStream2 {
    let controller_tokens: Vec<TokenStream2> = services
        .iter()
        .map(|controller_token| {
            let controller_name = controller_token.to_string();
            let binding = ROUTES.lock().expect("Failed to lock ROUTES in expand_controller_register");
            let controller: &Vec<String> =
                binding.get(&controller_name).expect(&format!("Failed to get controller {} in expand_controller_register", controller_name));
            let controller_ident = syn::Ident::new(&controller_name, Span::call_site().into());
            let router_path = controller
                .iter()
                .map(|name| {
                    let route_ident = syn::Ident::new(&format!("__route_{}", name), Span::call_site().into());

                    quote! {
                        ctx = t_controller.#route_ident(ctx, #module_name);
                    }
                })
                .collect::<Vec<TokenStream2>>();

            quote! {
                if ctx.register_controller(#module_name, #controller_name, Box::new(std::sync::Arc::new(#controller_ident::default()))) {
                    let t_controller = ctx.get_controller::<#controller_ident>(#module_name, #controller_name);
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

pub(crate) fn expand_service_register(module_name: String, services: &def::Array<def::Expr>) -> TokenStream2 {
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

pub(crate) fn expand_interceptor_register(module_name: String, services: &def::Array<def::Expr>) -> TokenStream2 {
    // println!("// gen_interceptor_register_tokens {} {:?}", module_name, services);
    let interceptor_tokens = services
        .iter()
        .map(|interceptor_token| {
            let interceptor_name = interceptor_token.to_string();
            let interceptor_ident = interceptor_token.to_token_stream();
            // let import_interceptor_tokens = import_path::gen_import_tokens(&interceptor_name);
            quote! {
                ctx.register_interceptor(#module_name, #interceptor_name, Box::new(std::sync::Arc::new(#interceptor_ident::default())));
            }
        })
        .collect::<Vec<TokenStream2>>();
    let interceptor_tokens = TokenStream2::from(quote! {
        #(#interceptor_tokens)*
    });
    return interceptor_tokens;
}

pub(crate) fn expand_imports_register(module_name: String, imports: &def::Array<def::Expr>, func: &ItemStruct) -> (TokenStream2, TokenStream2) {
    // let (impl_generics, ty_generics, where_clause) = func.generics.split_for_impl();

    let mut import_names = vec![];
    let imports = imports
        .iter()
        .map(|import_tokens| {
            let import_name = import_tokens.to_string();
            let import_tokens = import_tokens.to_token_stream();

            let import_call = syn::parse2::<ExprCall>(import_tokens.to_token_stream());
            if let Ok(import_call) = import_call {
                if let Expr::Path(path) = import_call.func.as_ref() {
                    let path_string = path.to_token_stream().to_string();
                    let mut import_module_ident = path_string.split("::").map(|item|item.trim()).collect::<Vec<_>>();
                    
                    if import_module_ident.len() > 2 {
                        import_module_ident.pop();
                    }else{
                        import_module_ident = vec![import_module_ident.first().unwrap()];
                    }

                    let dyn_module_name = import_module_ident.join("::");

                    let import_module_ident: Path = syn::parse_str(&dyn_module_name).expect(&format!("Failed to parse module path {} in expand_imports_register", dyn_module_name));
                    let import_module_ident = import_module_ident.to_token_stream();
                    import_names.push(dyn_module_name.clone());
        
                    quote! {
                        let mut dyn_module = #import_call;
                        let mut dyn_module_wrap = dyn_module.module.take().unwrap();
                        let mut dyn_module_services = dyn_module.services;
                        dyn_module_services.drain().for_each(|(k, v)| {
                            ctx.register_service(#dyn_module_name, &k, v);
                        });
                        let mut dyn_module_exports = dyn_module.exports;
                        ctx.append_exports(#dyn_module_name, dyn_module_exports, nidrs::get_meta_by_type::<#import_module_ident>().get_data::<nidrs::datasets::Global>().unwrap_or(&nidrs::datasets::Global(false)).value());
                        let mut ctx = dyn_module_wrap.init(ctx);
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

pub(crate) fn expand_dep_inject(con: &str, module_name: String, services: &def::Array<def::Expr>) -> TokenStream2 {
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

pub(crate) fn gen_service_inject_tokens(service_type: ServiceType, func: &ItemStruct) -> TokenStream2 {
    let (impl_generics, ty_generics, where_clause) = func.generics.split_for_impl();
    let is_service = service_type == ServiceType::Service;
    let is_interceptor = service_type == ServiceType::Interceptor;
    let service_type_indent = syn::Ident::new(service_type.into(), Span::call_site().into());
    let service_name_ident = func.ident.clone();

    let fields: Vec<TokenStream2> = if let syn::Fields::Named(fields) = &func.fields {
        fields
            .named
            .iter()
            .map(|field| {
                let field_ident = field.ident.as_ref().expect("Failed to get field identifier in gen_service_inject_tokens");
                let field_type = &field.ty;

                if let Type::Path(type_path) = field_type {
                    let type_ident =
                        type_path.path.segments.first().expect("Failed to get type identifier in gen_service_inject_tokens").ident.to_string();
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
            impl #impl_generics nidrs::#service_type_indent for #service_name_ident #ty_generics #where_clause  {}
        }
    };

    let meta_tokens = cmeta::CMeta::build_tokens();

    let inject_tokens = TokenStream2::from(quote! {
        #middle_tokens
        impl #impl_generics nidrs::Service for #service_name_ident #ty_generics #where_clause {
            fn inject(&self, ctx: nidrs::ModuleCtx, module_name: &str) -> nidrs::ModuleCtx{
                #(#fields)*
                ctx
            }
        }

        impl #impl_generics nidrs::ImplMeta for #service_name_ident #ty_generics #where_clause{
            fn __meta(&self) -> nidrs::InnerMeta {
                #meta_tokens
            }
        }
    });

    return inject_tokens;
}

pub(crate) fn expand_events_trigger(module_name: String, event_name: &str) -> TokenStream2 {
    // let event_name_ident = syn::Ident::new(event_name, Span::call_site().into());
    let binding = EVENTS.lock().expect("Failed to lock EVENTS in expand_events_trigger");
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

pub(crate) fn __service_derive(service_type: ServiceType, input: TokenStream) -> TokenStream {
    let func = parse_macro_input!(input as ItemStruct);

    println!("// service_derive {:?}", func.ident.to_string());

    let inject_tokens: TokenStream2 = gen_service_inject_tokens(service_type, &func);

    let derives_tokens = merge_derives(&func, &["Default"]);

    TokenStream::from(quote! {
        #(#derives_tokens)*
        #func

        #inject_tokens
    })
}

pub(crate) fn expand_exports_append(exports: &def::Array<def::Expr>) -> TokenStream2 {
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
        Vec::<&str>::from([#(#exports_names_tokens),*])
    });
    exports_names_tokens
}

pub(crate) fn merge_defaults_interceptors(interceptors: def::Array<def::Expr>) -> def::Array<def::Expr> {
    let defaults_interceptors = def::Array(
        DEFAULT_INTERS
            .lock()
            .expect("Failed to lock DEFAULT_INTERS in merge_defaults_interceptors")
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
