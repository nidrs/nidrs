#![feature(print_internals)]
#![feature(panic_internals)]
#![feature(alloc)]
#![feature(fmt_helpers_for_derive)]
#![allow(warnings, unused)]
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use axum::{routing::get, Router};
use nidrs::StateCtx;
mod app {
    use nidrs_macro::module;
    pub mod controller {
        use std::{collections::HashMap, sync::Arc};
        use axum::{
            extract::{Query, State},
            Json,
        };
        use nidrs::{Inject, StateCtx};
        use nidrs_macro::{controller, get, meta, post, uses};
        use super::service::AppService;
        pub struct AppController {
            app_service: Inject<AppService>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for AppController {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "AppController",
                    "app_service",
                    &&self.app_service,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for AppController {
            #[inline]
            fn default() -> AppController {
                AppController {
                    app_service: ::core::default::Default::default(),
                }
            }
        }
        impl nidrs::Controller for AppController {
            fn inject(
                &self,
                services: &std::sync::MutexGuard<
                    std::collections::HashMap<String, Box<dyn std::any::Any>>,
                >,
            ) {
                let service = services
                    .get("AppService")
                    .expect(
                        {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "[{0}] Service {1} not register.",
                                    "AppController",
                                    "AppService",
                                ),
                            );
                            res
                        }
                            .as_str(),
                    );
                let service = service
                    .downcast_ref::<std::sync::Arc<AppService>>()
                    .unwrap();
                self.app_service.inject(service.clone());
            }
        }
        impl AppController {
            pub async fn get_hello_world(
                &self,
                State(state): State<StateCtx>,
                Query(q): Query<HashMap<String, String>>,
            ) -> String {
                {
                    ::std::io::_print(format_args!("Query {0:?}\n", q));
                };
                self.app_service.get_hello_world()
            }
            pub fn __get_hello_world_meta(&self) -> HashMap<String, String> {
                let mut meta = HashMap::new();
                meta.insert("fun_name".to_string(), "get_hello_world".to_string());
                meta.insert("role".to_string(), "\"user\"".to_string());
                meta
            }
            pub async fn get_hello_world2(
                &self,
                State(state): State<StateCtx>,
                Query(q): Query<HashMap<String, String>>,
                Json(j): Json<serde_json::Value>,
            ) -> String {
                {
                    ::std::io::_print(format_args!("Query {0:?}\n", q));
                };
                {
                    ::std::io::_print(format_args!("Json {0:?}\n", j));
                };
                "Hello, World2!".to_string()
            }
        }
    }
    pub mod service {
        use nidrs::Inject;
        use nidrs_macro::{injectable, on_module_init};
        use crate::user::service::UserService;
        pub struct AppService {
            user_service: Inject<UserService>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for AppService {
            #[inline]
            fn clone(&self) -> AppService {
                AppService {
                    user_service: ::core::clone::Clone::clone(&self.user_service),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for AppService {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "AppService",
                    "user_service",
                    &&self.user_service,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for AppService {
            #[inline]
            fn default() -> AppService {
                AppService {
                    user_service: ::core::default::Default::default(),
                }
            }
        }
        impl nidrs::Service for AppService {
            fn inject(
                &self,
                services: &std::sync::MutexGuard<
                    std::collections::HashMap<String, Box<dyn std::any::Any>>,
                >,
            ) {
                let service = services
                    .get("UserService")
                    .expect(
                        {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "[{0}] Service {1} not register.",
                                    "AppService",
                                    "UserService",
                                ),
                            );
                            res
                        }
                            .as_str(),
                    );
                let service = service
                    .downcast_ref::<std::sync::Arc<UserService>>()
                    .unwrap();
                self.user_service.inject(service.clone());
            }
        }
        impl AppService {
            pub fn get_hello_world(&self) -> String {
                self.user_service.extract().get_hello_world()
            }
            pub fn get_hello_world2(&self) -> String {
                "Hello, nidrs2xx333!".to_string()
            }
        }
    }
    use controller::AppController;
    use service::AppService;
    use crate::user::UserModule;
    use crate::log::LogModule;
    use crate::conf::ConfModule;
    use crate::conf::ConfOptions;
    use crate::log::interceptor::LogInterceptor;
    pub struct AppModule;
    #[automatically_derived]
    impl ::core::clone::Clone for AppModule {
        #[inline]
        fn clone(&self) -> AppModule {
            AppModule
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AppModule {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "AppModule")
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for AppModule {
        #[inline]
        fn default() -> AppModule {
            AppModule {}
        }
    }
    impl nidrs::Module for AppModule {
        fn init(self, ctx: &nidrs::ModuleCtx) {
            use nidrs::{
                Service, Controller, Interceptor, HookCtx, InterceptorHook, ModuleCtx,
                StateCtx,
            };
            if ctx.modules.lock().unwrap().contains_key("AppModule") {
                return;
            }
            ctx.modules
                .lock()
                .unwrap()
                .insert(
                    "AppModule".to_string(),
                    Box::new(self) as Box<dyn std::any::Any>,
                );
            {
                ::std::io::_print(
                    format_args!(
                        "{0} ",
                        nidrs_extern::colored::Colorize::green("[nidrs]"),
                    ),
                );
            };
            {
                ::std::io::_print(
                    format_args!("Registering module {0}.\n", "AppModule"),
                );
            };
            {
                {
                    ::std::io::_print(
                        format_args!(
                            "{0} ",
                            nidrs_extern::colored::Colorize::green("[nidrs]"),
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!("Registering interceptor {0}.\n", "LogInterceptor"),
                    );
                };
                ctx.interceptors
                    .lock()
                    .unwrap()
                    .insert(
                        "LogInterceptor".to_string(),
                        Box::new(std::sync::Arc::new(LogInterceptor::default()))
                            as Box<dyn std::any::Any>,
                    );
                ctx.controllers
                    .lock()
                    .unwrap()
                    .insert(
                        "AppController".to_string(),
                        Box::new(
                            std::sync::Arc::new(controller::AppController::default()),
                        ),
                    );
                let controllers = ctx.controllers.lock().unwrap();
                let interceptors = ctx.interceptors.lock().unwrap();
                {
                    ::std::io::_print(
                        format_args!(
                            "{0} ",
                            nidrs_extern::colored::Colorize::green("[nidrs]"),
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!("Registering controller {0}.\n", "AppController"),
                    );
                };
                let t_controller = controllers.get("AppController").unwrap();
                let t_controller = t_controller
                    .downcast_ref::<std::sync::Arc<controller::AppController>>()
                    .unwrap();
                let t_controller = t_controller.clone();
                let t_interceptor_0 = interceptors.get("LogInterceptor").unwrap();
                let t_interceptor_0 = t_interceptor_0
                    .downcast_ref::<std::sync::Arc<LogInterceptor>>()
                    .unwrap();
                let t_interceptor_0 = t_interceptor_0.clone();
                {
                    ::std::io::_print(
                        format_args!(
                            "{0} ",
                            nidrs_extern::colored::Colorize::green("[nidrs]"),
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "Registering router \'{0} {1}\'.\n",
                            "get".to_uppercase(),
                            "/app/hello",
                        ),
                    );
                };
                ctx.routers
                    .lock()
                    .unwrap()
                    .push(
                        axum::Router::new()
                            .route(
                                "/app/hello",
                                axum::routing::get(|req, p0, p1| async move {
                                    let meta = t_controller.__get_hello_world_meta();
                                    let inter_ctx = nidrs::HookCtx {
                                        meta: meta,
                                        req: req,
                                    };
                                    t_interceptor_0.before(&inter_ctx).await;
                                    let r = t_controller.get_hello_world(p0, p1).await;
                                    t_interceptor_0.after(&inter_ctx).await;
                                    r
                                }),
                            ),
                    );
                let t_controller = controllers.get("AppController").unwrap();
                let t_controller = t_controller
                    .downcast_ref::<std::sync::Arc<controller::AppController>>()
                    .unwrap();
                let t_controller = t_controller.clone();
                {
                    ::std::io::_print(
                        format_args!(
                            "{0} ",
                            nidrs_extern::colored::Colorize::green("[nidrs]"),
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "Registering router \'{0} {1}\'.\n",
                            "post".to_uppercase(),
                            "/app/hello",
                        ),
                    );
                };
                ctx.routers
                    .lock()
                    .unwrap()
                    .push(
                        axum::Router::new()
                            .route(
                                "/app/hello",
                                axum::routing::post(|req, p0, p1, p2| async move {
                                    let meta = std::collections::HashMap::new();
                                    let inter_ctx = nidrs::HookCtx {
                                        meta: meta,
                                        req: req,
                                    };
                                    let r = t_controller.get_hello_world2(p0, p1, p2).await;
                                    r
                                }),
                            ),
                    );
                {
                    ::std::io::_print(
                        format_args!(
                            "{0} ",
                            nidrs_extern::colored::Colorize::green("[nidrs]"),
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!("Registering service {0}.\n", "AppService"),
                    );
                };
                ctx.services
                    .lock()
                    .unwrap()
                    .insert(
                        "AppService".to_string(),
                        Box::new(std::sync::Arc::new(AppService::default()))
                            as Box<dyn std::any::Any>,
                    );
            }
            {
                let dyn_module = ConfModule::for_root(ConfOptions {
                    log_level: "info".to_string(),
                });
                let mut dyn_module_services = dyn_module.services;
                for (k, v) in dyn_module_services.iter_mut() {
                    {
                        ::std::io::_print(
                            format_args!(
                                "{0} ",
                                nidrs_extern::colored::Colorize::green("[nidrs]"),
                            ),
                        );
                    };
                    {
                        ::std::io::_print(
                            format_args!("Registering dyn service {0}.\n", k),
                        );
                    };
                    ctx.services.lock().unwrap().insert(k.clone(), v.take().unwrap());
                }
                ConfModule::default().init(ctx);
                LogModule::default().init(ctx);
                UserModule::default().init(ctx);
            }
            {
                let services = ctx.services.lock().unwrap();
                let controllers = ctx.controllers.lock().unwrap();
                let interceptors = ctx.interceptors.lock().unwrap();
                let t = services.get("AppService").unwrap();
                let t = t.downcast_ref::<std::sync::Arc<AppService>>().unwrap();
                let t = t.clone();
                {
                    ::std::io::_print(
                        format_args!(
                            "{0} ",
                            nidrs_extern::colored::Colorize::green("[nidrs]"),
                        ),
                    );
                };
                {
                    ::std::io::_print(format_args!("Injecting {0}.\n", "AppService"));
                };
                t.inject(&services);
                let t = controllers.get("AppController").unwrap();
                let t = t.downcast_ref::<std::sync::Arc<AppController>>().unwrap();
                let t = t.clone();
                {
                    ::std::io::_print(
                        format_args!(
                            "{0} ",
                            nidrs_extern::colored::Colorize::green("[nidrs]"),
                        ),
                    );
                };
                {
                    ::std::io::_print(format_args!("Injecting {0}.\n", "AppController"));
                };
                t.inject(&services);
                let t = interceptors.get("LogInterceptor").unwrap();
                let t = t.downcast_ref::<std::sync::Arc<LogInterceptor>>().unwrap();
                let t = t.clone();
                {
                    ::std::io::_print(
                        format_args!(
                            "{0} ",
                            nidrs_extern::colored::Colorize::green("[nidrs]"),
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!("Injecting {0}.\n", "LogInterceptor"),
                    );
                };
                t.inject(&services);
            }
            {
                let services = ctx.services.lock().unwrap();
            }
        }
    }
}
mod conf {
    pub mod service {
        use nidrs::Inject;
        use nidrs_macro::{injectable, on_module_init};
        use super::options::ConfOptions;
        pub struct ConfService {
            pub options: Inject<ConfOptions>,
            pub log_level: String,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ConfService {
            #[inline]
            fn clone(&self) -> ConfService {
                ConfService {
                    options: ::core::clone::Clone::clone(&self.options),
                    log_level: ::core::clone::Clone::clone(&self.log_level),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ConfService {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "ConfService",
                    "options",
                    &self.options,
                    "log_level",
                    &&self.log_level,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for ConfService {
            #[inline]
            fn default() -> ConfService {
                ConfService {
                    options: ::core::default::Default::default(),
                    log_level: ::core::default::Default::default(),
                }
            }
        }
        impl nidrs::Service for ConfService {
            fn inject(
                &self,
                services: &std::sync::MutexGuard<
                    std::collections::HashMap<String, Box<dyn std::any::Any>>,
                >,
            ) {
                let service = services
                    .get("ConfOptions")
                    .expect(
                        {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "[{0}] Service {1} not register.",
                                    "ConfService",
                                    "ConfOptions",
                                ),
                            );
                            res
                        }
                            .as_str(),
                    );
                let service = service
                    .downcast_ref::<std::sync::Arc<ConfOptions>>()
                    .unwrap();
                self.options.inject(service.clone());
            }
        }
        impl ConfService {
            pub fn on_module_init(&self) {
                let options = self.options.extract();
                {
                    ::std::io::_print(
                        format_args!(
                            "ConfService initialized with log_level: {0:?}\n",
                            options,
                        ),
                    );
                };
            }
        }
    }
    pub mod options {
        use nidrs_macro::injectable;
        pub struct ConfOptions {
            pub log_level: String,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ConfOptions {
            #[inline]
            fn clone(&self) -> ConfOptions {
                ConfOptions {
                    log_level: ::core::clone::Clone::clone(&self.log_level),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ConfOptions {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "ConfOptions",
                    "log_level",
                    &&self.log_level,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for ConfOptions {
            #[inline]
            fn default() -> ConfOptions {
                ConfOptions {
                    log_level: ::core::default::Default::default(),
                }
            }
        }
        impl nidrs::Service for ConfOptions {
            fn inject(
                &self,
                services: &std::sync::MutexGuard<
                    std::collections::HashMap<String, Box<dyn std::any::Any>>,
                >,
            ) {}
        }
    }
    use std::{any::Any, collections::HashMap, sync::Arc};
    use nidrs::{DynamicModule, Service};
    use nidrs_macro::module;
    use service::ConfService;
    pub use options::ConfOptions;
    pub struct ConfModule;
    #[automatically_derived]
    impl ::core::clone::Clone for ConfModule {
        #[inline]
        fn clone(&self) -> ConfModule {
            ConfModule
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ConfModule {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "ConfModule")
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for ConfModule {
        #[inline]
        fn default() -> ConfModule {
            ConfModule {}
        }
    }
    impl nidrs::Module for ConfModule {
        fn init(self, ctx: &nidrs::ModuleCtx) {
            use nidrs::{
                Service, Controller, Interceptor, HookCtx, InterceptorHook, ModuleCtx,
                StateCtx,
            };
            if ctx.modules.lock().unwrap().contains_key("ConfModule") {
                return;
            }
            ctx.modules
                .lock()
                .unwrap()
                .insert(
                    "ConfModule".to_string(),
                    Box::new(self) as Box<dyn std::any::Any>,
                );
            {
                ::std::io::_print(
                    format_args!(
                        "{0} ",
                        nidrs_extern::colored::Colorize::green("[nidrs]"),
                    ),
                );
            };
            {
                ::std::io::_print(
                    format_args!("Registering module {0}.\n", "ConfModule"),
                );
            };
            {
                {
                    ::std::io::_print(
                        format_args!(
                            "{0} ",
                            nidrs_extern::colored::Colorize::green("[nidrs]"),
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!("Registering service {0}.\n", "ConfService"),
                    );
                };
                ctx.services
                    .lock()
                    .unwrap()
                    .insert(
                        "ConfService".to_string(),
                        Box::new(std::sync::Arc::new(ConfService::default()))
                            as Box<dyn std::any::Any>,
                    );
            }
            {}
            {
                let services = ctx.services.lock().unwrap();
                let controllers = ctx.controllers.lock().unwrap();
                let interceptors = ctx.interceptors.lock().unwrap();
                let t = services.get("ConfService").unwrap();
                let t = t.downcast_ref::<std::sync::Arc<ConfService>>().unwrap();
                let t = t.clone();
                {
                    ::std::io::_print(
                        format_args!(
                            "{0} ",
                            nidrs_extern::colored::Colorize::green("[nidrs]"),
                        ),
                    );
                };
                {
                    ::std::io::_print(format_args!("Injecting {0}.\n", "ConfService"));
                };
                t.inject(&services);
            }
            {
                let services = ctx.services.lock().unwrap();
                let service = services.get("ConfService").unwrap();
                let service = service
                    .downcast_ref::<std::sync::Arc<ConfService>>()
                    .unwrap();
                let service = service.clone();
                {
                    ::std::io::_print(
                        format_args!(
                            "{0} ",
                            nidrs_extern::colored::Colorize::green("[nidrs]"),
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "Triggering event on_module_init for {0}.\n",
                            "ConfService",
                        ),
                    );
                };
                service.on_module_init();
            }
        }
    }
    impl ConfModule {
        pub fn for_root(options: ConfOptions) -> DynamicModule {
            DynamicModule {
                services: HashMap::from([
                    (
                        "ConfOptions".to_string(),
                        Some(Box::new(Arc::new(options)) as Box<dyn Any + 'static>),
                    ),
                ]),
            }
        }
    }
}
mod user {
    use nidrs_macro::module;
    pub mod service {
        use std::sync::{Arc, Mutex};
        use nidrs::Inject;
        use nidrs_macro::injectable;
        use crate::app::service::AppService;
        pub struct UserService {
            app_service: Inject<AppService>,
            count: Arc<Mutex<i32>>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for UserService {
            #[inline]
            fn clone(&self) -> UserService {
                UserService {
                    app_service: ::core::clone::Clone::clone(&self.app_service),
                    count: ::core::clone::Clone::clone(&self.count),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for UserService {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "UserService",
                    "app_service",
                    &self.app_service,
                    "count",
                    &&self.count,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for UserService {
            #[inline]
            fn default() -> UserService {
                UserService {
                    app_service: ::core::default::Default::default(),
                    count: ::core::default::Default::default(),
                }
            }
        }
        impl nidrs::Service for UserService {
            fn inject(
                &self,
                services: &std::sync::MutexGuard<
                    std::collections::HashMap<String, Box<dyn std::any::Any>>,
                >,
            ) {
                let service = services
                    .get("AppService")
                    .expect(
                        {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "[{0}] Service {1} not register.",
                                    "UserService",
                                    "AppService",
                                ),
                            );
                            res
                        }
                            .as_str(),
                    );
                let service = service
                    .downcast_ref::<std::sync::Arc<AppService>>()
                    .unwrap();
                self.app_service.inject(service.clone());
            }
        }
        impl UserService {
            pub fn get_hello_world(&self) -> String {
                self.app_service.extract().get_hello_world2()
            }
            pub fn get_hello_world2(&self) -> String {
                let mut count = self.count.lock().unwrap();
                *count += 1;
                {
                    let res = ::alloc::fmt::format(
                        format_args!("Hello, World! {0}", count),
                    );
                    res
                }
            }
        }
    }
    pub mod controller {
        use std::collections::HashMap;
        use axum::{
            extract::{Query, State},
            Json,
        };
        use nidrs::{Inject, StateCtx};
        use nidrs_macro::{controller, get, post};
        use super::service::UserService;
        pub struct UserController {
            user_service: Inject<UserService>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for UserController {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "UserController",
                    "user_service",
                    &&self.user_service,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for UserController {
            #[inline]
            fn default() -> UserController {
                UserController {
                    user_service: ::core::default::Default::default(),
                }
            }
        }
        impl nidrs::Controller for UserController {
            fn inject(
                &self,
                services: &std::sync::MutexGuard<
                    std::collections::HashMap<String, Box<dyn std::any::Any>>,
                >,
            ) {
                let service = services
                    .get("UserService")
                    .expect(
                        {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "[{0}] Service {1} not register.",
                                    "UserController",
                                    "UserService",
                                ),
                            );
                            res
                        }
                            .as_str(),
                    );
                let service = service
                    .downcast_ref::<std::sync::Arc<UserService>>()
                    .unwrap();
                self.user_service.inject(service.clone());
            }
        }
        impl UserController {
            pub async fn get_hello_world(
                &self,
                Query(q): Query<HashMap<String, String>>,
            ) -> String {
                {
                    ::std::io::_print(format_args!("Query {0:?}\n", q));
                };
                self.user_service.extract().get_hello_world2()
            }
        }
    }
    use service::UserService;
    use controller::UserController;
    use crate::app::AppModule;
    pub struct UserModule;
    #[automatically_derived]
    impl ::core::clone::Clone for UserModule {
        #[inline]
        fn clone(&self) -> UserModule {
            UserModule
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for UserModule {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "UserModule")
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for UserModule {
        #[inline]
        fn default() -> UserModule {
            UserModule {}
        }
    }
    impl nidrs::Module for UserModule {
        fn init(self, ctx: &nidrs::ModuleCtx) {
            use nidrs::{
                Service, Controller, Interceptor, HookCtx, InterceptorHook, ModuleCtx,
                StateCtx,
            };
            if ctx.modules.lock().unwrap().contains_key("UserModule") {
                return;
            }
            ctx.modules
                .lock()
                .unwrap()
                .insert(
                    "UserModule".to_string(),
                    Box::new(self) as Box<dyn std::any::Any>,
                );
            {
                ::std::io::_print(
                    format_args!(
                        "{0} ",
                        nidrs_extern::colored::Colorize::green("[nidrs]"),
                    ),
                );
            };
            {
                ::std::io::_print(
                    format_args!("Registering module {0}.\n", "UserModule"),
                );
            };
            {
                ctx.controllers
                    .lock()
                    .unwrap()
                    .insert(
                        "UserController".to_string(),
                        Box::new(
                            std::sync::Arc::new(controller::UserController::default()),
                        ),
                    );
                let controllers = ctx.controllers.lock().unwrap();
                let interceptors = ctx.interceptors.lock().unwrap();
                {
                    ::std::io::_print(
                        format_args!(
                            "{0} ",
                            nidrs_extern::colored::Colorize::green("[nidrs]"),
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!("Registering controller {0}.\n", "UserController"),
                    );
                };
                let t_controller = controllers.get("UserController").unwrap();
                let t_controller = t_controller
                    .downcast_ref::<std::sync::Arc<controller::UserController>>()
                    .unwrap();
                let t_controller = t_controller.clone();
                {
                    ::std::io::_print(
                        format_args!(
                            "{0} ",
                            nidrs_extern::colored::Colorize::green("[nidrs]"),
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "Registering router \'{0} {1}\'.\n",
                            "get".to_uppercase(),
                            "/user/hello",
                        ),
                    );
                };
                ctx.routers
                    .lock()
                    .unwrap()
                    .push(
                        axum::Router::new()
                            .route(
                                "/user/hello",
                                axum::routing::get(|req, p0| async move {
                                    let meta = std::collections::HashMap::new();
                                    let inter_ctx = nidrs::HookCtx {
                                        meta: meta,
                                        req: req,
                                    };
                                    let r = t_controller.get_hello_world(p0).await;
                                    r
                                }),
                            ),
                    );
                {
                    ::std::io::_print(
                        format_args!(
                            "{0} ",
                            nidrs_extern::colored::Colorize::green("[nidrs]"),
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!("Registering service {0}.\n", "UserService"),
                    );
                };
                ctx.services
                    .lock()
                    .unwrap()
                    .insert(
                        "UserService".to_string(),
                        Box::new(std::sync::Arc::new(UserService::default()))
                            as Box<dyn std::any::Any>,
                    );
            }
            {
                AppModule::default().init(ctx);
            }
            {
                let services = ctx.services.lock().unwrap();
                let controllers = ctx.controllers.lock().unwrap();
                let interceptors = ctx.interceptors.lock().unwrap();
                let t = services.get("UserService").unwrap();
                let t = t.downcast_ref::<std::sync::Arc<UserService>>().unwrap();
                let t = t.clone();
                {
                    ::std::io::_print(
                        format_args!(
                            "{0} ",
                            nidrs_extern::colored::Colorize::green("[nidrs]"),
                        ),
                    );
                };
                {
                    ::std::io::_print(format_args!("Injecting {0}.\n", "UserService"));
                };
                t.inject(&services);
                let t = controllers.get("UserController").unwrap();
                let t = t.downcast_ref::<std::sync::Arc<UserController>>().unwrap();
                let t = t.clone();
                {
                    ::std::io::_print(
                        format_args!(
                            "{0} ",
                            nidrs_extern::colored::Colorize::green("[nidrs]"),
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!("Injecting {0}.\n", "UserController"),
                    );
                };
                t.inject(&services);
            }
            {
                let services = ctx.services.lock().unwrap();
            }
        }
    }
}
mod log {
    use nidrs_macro::module;
    pub mod service {
        use nidrs_macro::injectable;
        pub struct LogService {}
        #[automatically_derived]
        impl ::core::default::Default for LogService {
            #[inline]
            fn default() -> LogService {
                LogService {}
            }
        }
        impl nidrs::Service for LogService {
            fn inject(
                &self,
                services: &std::sync::MutexGuard<
                    std::collections::HashMap<String, Box<dyn std::any::Any>>,
                >,
            ) {}
        }
        impl LogService {
            pub fn log(&self, msg: &str) {
                {
                    ::std::io::_print(format_args!("[Log] {0}\n", msg));
                };
            }
        }
    }
    pub mod interceptor {
        use nidrs::{Inject, Interceptor, HookCtx, InterceptorHook};
        use nidrs_macro::interceptor;
        use super::service::LogService;
        pub struct LogInterceptor {
            log_service: Inject<LogService>,
        }
        #[automatically_derived]
        impl ::core::default::Default for LogInterceptor {
            #[inline]
            fn default() -> LogInterceptor {
                LogInterceptor {
                    log_service: ::core::default::Default::default(),
                }
            }
        }
        impl nidrs::Interceptor for LogInterceptor {
            fn inject(
                &self,
                services: &std::sync::MutexGuard<
                    std::collections::HashMap<String, Box<dyn std::any::Any>>,
                >,
            ) {
                let service = services
                    .get("LogService")
                    .expect(
                        {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "[{0}] Service {1} not register.",
                                    "LogInterceptor",
                                    "LogService",
                                ),
                            );
                            res
                        }
                            .as_str(),
                    );
                let service = service
                    .downcast_ref::<std::sync::Arc<LogService>>()
                    .unwrap();
                self.log_service.inject(service.clone());
            }
        }
        impl InterceptorHook for LogInterceptor {
            async fn before(&self, _ctx: &HookCtx) {
                {
                    ::std::io::_print(format_args!("ctx: {0:?}\n", _ctx));
                };
                self.log_service.log("Before");
            }
            async fn after(&self, _ctx: &HookCtx) {
                self.log_service.log("After");
            }
        }
    }
    use service::LogService;
    use interceptor::LogInterceptor;
    pub struct LogModule;
    #[automatically_derived]
    impl ::core::clone::Clone for LogModule {
        #[inline]
        fn clone(&self) -> LogModule {
            LogModule
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for LogModule {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "LogModule")
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for LogModule {
        #[inline]
        fn default() -> LogModule {
            LogModule {}
        }
    }
    impl nidrs::Module for LogModule {
        fn init(self, ctx: &nidrs::ModuleCtx) {
            use nidrs::{
                Service, Controller, Interceptor, HookCtx, InterceptorHook, ModuleCtx,
                StateCtx,
            };
            if ctx.modules.lock().unwrap().contains_key("LogModule") {
                return;
            }
            ctx.modules
                .lock()
                .unwrap()
                .insert(
                    "LogModule".to_string(),
                    Box::new(self) as Box<dyn std::any::Any>,
                );
            {
                ::std::io::_print(
                    format_args!(
                        "{0} ",
                        nidrs_extern::colored::Colorize::green("[nidrs]"),
                    ),
                );
            };
            {
                ::std::io::_print(
                    format_args!("Registering module {0}.\n", "LogModule"),
                );
            };
            {
                {
                    ::std::io::_print(
                        format_args!(
                            "{0} ",
                            nidrs_extern::colored::Colorize::green("[nidrs]"),
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!("Registering service {0}.\n", "LogService"),
                    );
                };
                ctx.services
                    .lock()
                    .unwrap()
                    .insert(
                        "LogService".to_string(),
                        Box::new(std::sync::Arc::new(LogService::default()))
                            as Box<dyn std::any::Any>,
                    );
            }
            {}
            {
                let services = ctx.services.lock().unwrap();
                let controllers = ctx.controllers.lock().unwrap();
                let interceptors = ctx.interceptors.lock().unwrap();
                let t = services.get("LogService").unwrap();
                let t = t.downcast_ref::<std::sync::Arc<LogService>>().unwrap();
                let t = t.clone();
                {
                    ::std::io::_print(
                        format_args!(
                            "{0} ",
                            nidrs_extern::colored::Colorize::green("[nidrs]"),
                        ),
                    );
                };
                {
                    ::std::io::_print(format_args!("Injecting {0}.\n", "LogService"));
                };
                t.inject(&services);
            }
            {
                let services = ctx.services.lock().unwrap();
            }
        }
    }
}
fn main() {
    let mut app = nidrs::NidrsFactory::create(app::AppModule);
    app
        .router = app
        .router
        .merge(
            Router::<StateCtx>::new().route("/api", get(|| async { "Hello, World!" })),
        );
    let app = app.listen::<AppError>(3000);
    let _ = tokio::runtime::Runtime::new().unwrap().block_on(app);
}
pub struct AppState {}
#[automatically_derived]
impl ::core::clone::Clone for AppState {
    #[inline]
    fn clone(&self) -> AppState {
        AppState {}
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for AppState {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "AppState")
    }
}
#[automatically_derived]
impl ::core::default::Default for AppState {
    #[inline]
    fn default() -> AppState {
        AppState {}
    }
}
pub enum AppError {}
impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        ::core::panicking::panic("not implemented")
    }
}
extern crate alloc;
