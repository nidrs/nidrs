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
use axum::{http::StatusCode, response::IntoResponse, routing::get, Form, Router};
use nidrs::{Exception, StateCtx};
use nidrs_extern::colored::Colorize;
mod app {
    use nidrs_macro::module;
    pub mod controller {
        use std::{collections::HashMap, sync::Arc};
        use axum::{
            extract::{Query, State},
            http::StatusCode, Json,
        };
        use nidrs::{throw, Exception, Inject, StateCtx};
        use nidrs_macro::{controller, get, meta, post, uses};
        use crate::{shared::fn_test::fn_test, AppError, AppResult};
        use super::{dto::Status, service::AppService};
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
        impl nidrs::ControllerService for AppController {
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
            pub fn __meta(&self) -> HashMap<String, String> {
                let mut meta = HashMap::new();
                meta.insert("struct_name".to_string(), "AppController".to_string());
                meta.insert("auth".to_string(), "\"true\"".to_string());
                meta.insert("role".to_string(), "\"admin\"".to_string());
                meta
            }
        }
        impl AppController {
            pub async fn get_hello_world(
                &self,
                Query(q): Query<HashMap<String, String>>,
            ) -> AppResult<Status> {
                {
                    ::std::io::_print(format_args!("Query {0:?}\n", q));
                };
                Ok(Status {
                    db: "ok".to_string(),
                    redis: "ok".to_string(),
                })
            }
            pub fn __get_hello_world_meta(&self) -> HashMap<String, String> {
                let mut meta = HashMap::new();
                meta.insert("fun_name".to_string(), "get_hello_world".to_string());
                meta.insert("role".to_string(), "\"user\"".to_string());
                meta
            }
            pub async fn get_hello_world2(
                &self,
                Query(q): Query<HashMap<String, String>>,
            ) -> AppResult<String> {
                {
                    ::std::io::_print(format_args!("Query {0:?}\n", q));
                };
                Ok(self.app_service.get_hello_world())
            }
            pub async fn post_hello_world(
                &self,
                Query(q): Query<HashMap<String, String>>,
                Json(j): Json<serde_json::Value>,
            ) -> AppResult<String> {
                {
                    ::std::io::_print(format_args!("Query {0:?}\n", q));
                };
                {
                    ::std::io::_print(format_args!("Json {0:?}\n", j));
                };
                Ok("Hello, World2!".to_string())
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
    pub mod dto {
        use axum::{
            body::{Body, Bytes},
            http::{header, StatusCode},
            response::{IntoResponse, Response},
        };
        use nidrs::AnyBody;
        use serde::{Deserialize, Serialize};
        pub struct Status {
            pub db: String,
            pub redis: String,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Status {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "Status",
                        false as usize + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "db",
                        &self.db,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "redis",
                        &self.redis,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Status {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "db" => _serde::__private::Ok(__Field::__field0),
                                "redis" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"db" => _serde::__private::Ok(__Field::__field0),
                                b"redis" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Status>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Status;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Status",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Status with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Status with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Status {
                                db: __field0,
                                redis: __field1,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("db"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("redis"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("db")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("redis")?
                                }
                            };
                            _serde::__private::Ok(Status {
                                db: __field0,
                                redis: __field1,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["db", "redis"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Status",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Status>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for Status {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Status",
                    "db",
                    &self.db,
                    "redis",
                    &&self.redis,
                )
            }
        }
        impl IntoResponse for Status {
            fn into_response(self) -> Response {
                let json_body = match serde_json::to_string(&self) {
                    Ok(json) => json,
                    Err(_) => {
                        return Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body("Internal server error".into())
                            .unwrap();
                    }
                };
                let res: Response<Body> = Response::builder()
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(json_body.into())
                    .unwrap();
                res
            }
        }
    }
    pub mod exception {
        use nidrs::Exception;
        pub enum AppException {
            ServiceException,
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
                Service, ControllerService, InterceptorService, InterCtx, Interceptor, ModuleCtx,
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
                let t_interceptor_1 = interceptors.get("LogInterceptor").unwrap();
                let t_interceptor_1 = t_interceptor_1
                    .downcast_ref::<std::sync::Arc<LogInterceptor>>()
                    .unwrap();
                let t_interceptor_1 = t_interceptor_1.clone();
                let meta = std::collections::HashMap::<String, String>::new();
                let mut t_meta = t_controller.__meta();
                t_meta.extend(meta);
                let meta = t_meta;
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
                let router = axum::Router::new()
                    .route(
                        "/app/hello",
                        axum::routing::post(|parts, p0, p1| async move {
                            let t_body = p1;
                            let ctx = InterCtx {
                                meta: meta.clone(),
                                parts,
                                body: t_body,
                            };
                            let t_inter_fn_0 = |ctx: InterCtx<_>| async move {
                                let t_body = ctx.body;
                                t_controller.post_hello_world(p0, t_body).await
                            };
                            let t_inter_fn_1 = |ctx: InterCtx<_>| async move {
                                t_interceptor_0.interceptor(ctx, t_inter_fn_0).await
                            };
                            t_interceptor_1.interceptor(ctx, t_inter_fn_1).await
                        }),
                    );
                ctx.routers.lock().unwrap().push(router);
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
                let meta = t_controller.__get_hello_world_meta();
                let mut t_meta = t_controller.__meta();
                t_meta.extend(meta);
                let meta = t_meta;
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
                let router = axum::Router::new()
                    .route(
                        "/app/hello",
                        axum::routing::get(|parts, p0| async move {
                            let t_body = nidrs_extern::axum::body::Body::empty();
                            let ctx = InterCtx {
                                meta: meta.clone(),
                                parts,
                                body: t_body,
                            };
                            let t_inter_fn_0 = |ctx: InterCtx<_>| async move {
                                t_controller.get_hello_world(p0).await
                            };
                            t_interceptor_0.interceptor(ctx, t_inter_fn_0).await
                        }),
                    );
                ctx.routers.lock().unwrap().push(router);
                let t_controller = controllers.get("AppController").unwrap();
                let t_controller = t_controller
                    .downcast_ref::<std::sync::Arc<controller::AppController>>()
                    .unwrap();
                let t_controller = t_controller.clone();
                let meta = std::collections::HashMap::<String, String>::new();
                let mut t_meta = t_controller.__meta();
                t_meta.extend(meta);
                let meta = t_meta;
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
                            "/app/hello2",
                        ),
                    );
                };
                let router = axum::Router::new()
                    .route(
                        "/app/hello2",
                        axum::routing::get(|p0| async move {
                            t_controller.get_hello_world2(p0).await
                        }),
                    );
                ctx.routers.lock().unwrap().push(router);
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
                Service, ControllerService, InterceptorService, InterCtx, Interceptor, ModuleCtx,
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
        impl nidrs::ControllerService for UserController {
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
                Service, ControllerService, InterceptorService, InterCtx, Interceptor, ModuleCtx,
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
                let meta = std::collections::HashMap::<String, String>::new();
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
                let router = axum::Router::new()
                    .route(
                        "/user/hello",
                        axum::routing::get(|p0| async move {
                            t_controller.get_hello_world(p0).await
                        }),
                    );
                ctx.routers.lock().unwrap().push(router);
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
        use std::fmt::Debug;
        use axum::{
            body::Body, extract::FromRequest,
            http::{HeaderName, HeaderValue, StatusCode},
            response::{IntoResponse, IntoResponseParts, Response, ResponseParts},
            Json,
        };
        use axum_extra::headers::Header;
        use nidrs::{
            AnyBody, Exception, InterCtx, Inject, InterceptorService, Interceptor,
            IntoAnyResponse, StateCtx,
        };
        use nidrs_macro::interceptor;
        use serde::{de::DeserializeOwned, Serialize, Serializer};
        use crate::{app::dto::Status, AppError, AppResult};
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
        impl nidrs::InterceptorService for LogInterceptor {
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
        impl<B: FromRequest<StateCtx> + Debug, P: IntoAnyResponse> Interceptor<B, P>
        for LogInterceptor {
            type R = AnyBody;
            async fn interceptor<F, H>(
                &self,
                ctx: InterCtx<B>,
                handler: H,
            ) -> AppResult<Self::R>
            where
                F: std::future::Future<Output = AppResult<P>> + Send + 'static,
                H: FnOnce(InterCtx<B>) -> F,
            {
                {
                    ::std::io::_print(format_args!("ctx: {0:?}\n", ctx));
                };
                self.log_service.log("Before");
                let r: AppResult<AnyBody> = handler(ctx)
                    .await
                    .map(|r| IntoAnyResponse::from_serializable(r));
                self.log_service.log("After");
                r
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
                Service, ControllerService, InterceptorService, InterCtx, Interceptor, ModuleCtx,
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
mod shared {
    pub mod fn_test {
        use axum::http::StatusCode;
        use nidrs::{throw, Exception};
        use crate::{AppError, AppResult};
        pub fn fn_test() -> AppResult {
            nidrs::__throw(
                Exception::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    anyhow::Error::msg("Error"),
                ),
                &{
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "from {0} line {1}",
                            "example/src/shared/fn_test.rs",
                            7usize,
                        ),
                    );
                    res
                },
            )?;
            Ok(())
        }
    }
}
pub use nidrs::AppResult;
pub use nidrs::AppError;
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
extern crate alloc;
