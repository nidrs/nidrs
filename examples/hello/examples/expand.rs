#![feature(print_internals)]
#![feature(panic_internals)]
#![feature(alloc)]
#![feature(fmt_helpers_for_derive)]
#![allow(warnings, unused)]
#![feature(hint_must_use)]
#![feature(liballoc_internals)]
// >>Push: Global("app") -- [None]
//  CMETA: []
// >>Push: Module("AppModule") -- [None]
// >>Push: Service("AppController") -- [Some(String("AppModule"))]
//  CMETA: ["version"]
//  CMETA: ["role", "auth"]
//  CMETA: ["ServiceType"]
//  CMETA: ["ServiceName"]
//  CMETA: ["ControllerPath"]
// service_derive "AppController"
// >>Push: Handler("get_hello_world") -- [Some(String("AppModule"))]
//  CMETA: ["RouterOut", "RouterIn"]
//  CMETA: ["method_uses"]
//  CMETA: ["arr"]
//  CMETA: ["DisableDefaultPrefix"]
//  CMETA: ["version"]
//  CMETA: ["RouterName"]
//  CMETA: ["RouterMethod"]
//  CMETA: ["RouterPath"]
// route_derive "get_hello_world"
// << Pop: Some(Handler("get_hello_world")) ["RouterOut", "version", "RouterMethod", "arr", "method_uses", "RouterPath", "RouterName", "handler", "DisableDefaultPrefix", "RouterIn", "service", "role", "ServiceName", "ControllerPath", "ServiceType", "auth", "module", "global"]

// >>Push: Handler("get_hello_world2") -- [Some(String("AppModule"))]
//  CMETA: ["RouterIn", "RouterOut"]
//  CMETA: ["method_uses"]
//  CMETA: ["arr"]
//  CMETA: ["DisableDefaultPrefix"]
//  CMETA: ["version"]
//  CMETA: ["RouterName"]
//  CMETA: ["RouterMethod"]
//  CMETA: ["RouterPath"]
// route_derive "get_hello_world2"
// << Pop: Some(Handler("get_hello_world2")) ["method_uses", "RouterIn", "arr", "DisableDefaultPrefix", "RouterMethod", "RouterOut", "RouterPath", "RouterName", "handler", "version", "service", "role", "ServiceName", "ControllerPath", "ServiceType", "auth", "module", "global"]

// >>Push: Handler("post_hello_world") -- [Some(String("AppModule"))]
//  CMETA: ["RouterIn", "RouterOut"]
//  CMETA: ["RouterName"]
//  CMETA: ["RouterMethod"]
//  CMETA: ["RouterPath"]
// route_derive "post_hello_world"
// << Pop: Some(Handler("post_hello_world")) ["RouterMethod", "RouterOut", "RouterPath", "RouterName", "RouterIn", "handler", "service", "role", "ServiceName", "version", "ControllerPath", "ServiceType", "auth", "module", "global"]

// << Pop: Some(Service("AppController")) ["service", "role", "ServiceName", "version", "ControllerPath", "ServiceType", "auth", "module", "global"]

// >>Push: Service("AppInterceptor") -- [Some(String("AppModule"))]
//  CMETA: ["ServiceType"]
//  CMETA: ["ServiceName"]
// service_derive "AppInterceptor"
// << Pop: Some(Service("AppInterceptor")) ["ServiceName", "service", "ServiceType", "module", "global"]

// >>Push: Service("AppService") -- [Some(String("AppModule"))]
//  CMETA: ["test"]
//  CMETA: ["test2"]
//  CMETA: ["ServiceType"]
//  CMETA: ["ServiceName"]
// service_derive "AppService"
// << Pop: Some(Service("AppService")) ["ServiceName", "service", "test", "test2", "ServiceType", "module", "global"]

// >>Push: Service("AppModule") -- [Some(String("AppModule"))]
//  CMETA: ["__"]
// module "AppModule"
// << Pop: Some(Service("AppModule")) ["service", "__", "module", "global"]

// << Pop: Some(Module("AppModule")) ["module", "global"]

// >>Push: Module("ConfModule") -- [None]
// >>Push: Service("ConfOptions") -- [Some(String("ConfModule"))]
//  CMETA: ["ServiceType"]
//  CMETA: ["ServiceName"]
// service_derive "ConfOptions"
// << Pop: Some(Service("ConfOptions")) ["service", "ServiceType", "ServiceName", "module", "global"]

// >>Push: Service("ConfService") -- [Some(String("ConfModule"))]
//  CMETA: ["ServiceType"]
//  CMETA: ["ServiceName"]
// service_derive "ConfService"
// << Pop: Some(Service("ConfService")) ["ServiceType", "ServiceName", "service", "module", "global"]

// >>Push: Service("ConfModule") -- [Some(String("ConfModule"))]
//  CMETA: ["__"]
// module "ConfModule"
// << Pop: Some(Service("ConfModule")) ["__", "service", "module", "global"]

// << Pop: Some(Module("ConfModule")) ["module", "global"]

// >>Push: Module("UserModule") -- [None]
// >>Push: Service("UserController") -- [Some(String("UserModule"))]
//  CMETA: ["ServiceType"]
//  CMETA: ["ServiceName"]
//  CMETA: ["ControllerPath"]
// service_derive "UserController"
// >>Push: Handler("get_all") -- [Some(String("UserModule"))]
//  CMETA: ["RouterOut", "RouterIn"]
//  CMETA: ["RouterName"]
//  CMETA: ["RouterMethod"]
//  CMETA: ["RouterPath"]
// route_derive "get_all"
// << Pop: Some(Handler("get_all")) ["RouterPath", "RouterOut", "RouterIn", "handler", "RouterMethod", "RouterName", "ServiceType", "ControllerPath", "service", "ServiceName", "module", "global"]

// >>Push: Handler("get_one") -- [Some(String("UserModule"))]
//  CMETA: ["RouterIn", "RouterOut"]
//  CMETA: ["RouterName"]
//  CMETA: ["RouterMethod"]
//  CMETA: ["RouterPath"]
// route_derive "get_one"
// << Pop: Some(Handler("get_one")) ["RouterOut", "RouterMethod", "handler", "RouterIn", "RouterName", "RouterPath", "ServiceType", "ControllerPath", "service", "ServiceName", "module", "global"]

// >>Push: Handler("create_user") -- [Some(String("UserModule"))]
//  CMETA: ["RouterOut", "RouterIn"]
//  CMETA: ["RouterName"]
//  CMETA: ["RouterMethod"]
//  CMETA: ["RouterPath"]
// route_derive "create_user"
// << Pop: Some(Handler("create_user")) ["RouterOut", "RouterIn", "RouterPath", "RouterName", "handler", "RouterMethod", "ServiceType", "ControllerPath", "service", "ServiceName", "module", "global"]

// << Pop: Some(Service("UserController")) ["ServiceType", "ControllerPath", "service", "ServiceName", "module", "global"]

// >>Push: Service("UserService") -- [Some(String("UserModule"))]
//  CMETA: ["ServiceType"]
//  CMETA: ["ServiceName"]
// service_derive "UserService"
// << Pop: Some(Service("UserService")) ["service", "ServiceType", "ServiceName", "module", "global"]

// >>Push: Service("UserModule") -- [Some(String("UserModule"))]
//  CMETA: ["__"]
// module "UserModule"
// << Pop: Some(Service("UserModule")) ["__", "service", "module", "global"]
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod app {
    use nidrs::macros::module;
    pub mod controller {
        use super::{
            dto::{ArgDto, Status},
            interceptor::AppInterceptor,
            service::AppService,
        };
        use crate::app::dto::Mongo;
        use crate::AppResult;
        use nidrs::externs::axum::{extract::Query, response::AppendHeaders, Json};
        use nidrs::macros::{controller, get, meta, post, uses};
        use nidrs::openapi::api;
        use nidrs::{version, Inject, Meta};
        use std::collections::HashMap;
        pub struct AppController {
            app_service: Inject<AppService>,
        }
        #[automatically_derived]
        impl ::core::default::Default for AppController {
            #[inline]
            fn default() -> AppController {
                AppController { app_service: ::core::default::Default::default() }
            }
        }
        impl nidrs::Controller for AppController {}
        impl nidrs::Service for AppController {
            fn inject(&self, ctx: nidrs::ModuleCtx, module_name: &str) -> nidrs::ModuleCtx {
                let service = ctx.get_service::<AppService>(&module_name, "AppService");
                self.app_service.inject(service.clone());
                ctx
            }
        }
        impl nidrs::ImplMeta for AppController {
            fn __meta(&self) -> nidrs::InnerMeta {
                let mut meta = nidrs::InnerMeta::new();
                meta.set("service", "AppController");
                meta.set("role", "admin");
                meta.set_data(nidrs::datasets::ServiceName::from("AppController"));
                meta.set("version", "v1");
                meta.set_data(nidrs::datasets::ControllerPath::from(""));
                meta.set_data(nidrs::datasets::ServiceType::from("Controller"));
                meta.set("auth", "true");
                meta.set("module", "AppModule");
                meta.set("global", "app");
                meta
            }
        }
        impl AppController {
            pub async fn get_hello_world(
                &self,
                meta: Meta,
                Query(q): Query<HashMap<String, String>>,
            ) -> AppResult<(AppendHeaders<[(String, String); 2]>, Json<Status>)> {
                {
                    ::std::io::_print(format_args!("Query {0:?}\n", q));
                };
                {
                    ::std::io::_print(format_args!("Meta Keys {0:?}\n", meta.keys()));
                };
                {
                    ::std::io::_print(format_args!("Meta {0:?}\n", meta.get::<&str>("role")));
                };
                {
                    ::std::io::_print(format_args!("Meta {0:?}\n", meta.get_data::<nidrs::datasets::DisableDefaultPrefix>(),));
                };
                Ok((
                    AppendHeaders([("X-Custom-Header".to_string(), "hello".to_string()), ("X-Custom-Header".to_string(), "world".to_string())]),
                    Json(Status { db: "ok".to_string(), redis: "ok".to_string(), mongo: Mongo { count: 100 } }),
                ))
            }
            pub fn __meta_get_hello_world(&self) -> nidrs::InnerMeta {
                let mut meta = nidrs::InnerMeta::new();
                meta.set_data(nidrs::openapi::RouterOut(
                    nidrs::openapi::RouterParams::default().comb::<AppResult<(AppendHeaders<[(String, String); 2]>, Json<Status>)>>(""),
                ));
                meta.set("version", "v2");
                meta.set_data(nidrs::datasets::RouterMethod::from("get"));
                meta.set("arr", ["user"]);
                meta.set("method_uses", ["AppInterceptor"]);
                meta.set_data(nidrs::datasets::RouterPath::from("/hello"));
                meta.set_data(nidrs::datasets::RouterName::from("get_hello_world"));
                meta.set("handler", "get_hello_world");
                meta.set_data(nidrs::datasets::DisableDefaultPrefix(false));
                meta.set_data(nidrs::openapi::RouterIn(
                    nidrs::openapi::RouterParams::default().comb::<Meta>("meta").comb::<Query<HashMap<String, String>>>(""),
                ));
                meta.set("service", "AppController");
                meta.set("role", "admin");
                meta.set_data(nidrs::datasets::ServiceName::from("AppController"));
                meta.set_data(nidrs::datasets::ControllerPath::from(""));
                meta.set_data(nidrs::datasets::ServiceType::from("Controller"));
                meta.set("auth", "true");
                meta.set("module", "AppModule");
                meta.set("global", "app");
                meta
            }
            pub fn __route_get_hello_world(&self, mut ctx: nidrs::ModuleCtx, module: &str) -> nidrs::ModuleCtx {
                use axum::response::IntoResponse;
                use nidrs::externs::axum;
                use nidrs::externs::axum::{extract::Query, Json};
                use nidrs::externs::meta::{InnerMeta, Meta};
                use nidrs::Interceptor;
                use serde_json::Value;
                let mut meta = self.__meta_get_hello_world();
                let router_info = ctx.get_router_full(&meta);
                if let Err(e) = router_info {
                    {
                        ::core::panicking::panic_fmt(format_args!("[{0}] {1:?}", "__route_get_hello_world", e));
                    };
                }
                let full_path = router_info.unwrap();
                {
                    ::std::io::_print(format_args!("{0} ", nidrs_extern::colored::Colorize::green("[nidrs]"),));
                };
                {
                    ::std::io::_print(format_args!("Registering router \'{0} {1}\'.\n", "get".to_uppercase(), full_path,));
                };
                meta.set_data(nidrs::datasets::RouterFullPath(full_path.clone()));
                let meta = Meta::new(meta);
                let controller_name = meta.get_data::<nidrs::datasets::ServiceName>().unwrap().value();
                let t_controller = ctx.get_controller::<Self>(module, controller_name);
                let router = nidrs::externs::axum::Router::new()
                    .route(
                        &full_path,
                        nidrs::externs::axum::routing::get(|p0, p1| async move {
                            let r = t_controller.get_hello_world(p0, p1).await;
                            r
                        }),
                    )
                    .route_layer(nidrs::externs::axum::Extension(meta.clone()))
                    .layer(axum::middleware::from_fn({
                        let inter = ctx.get_interceptor::<AppInterceptor>(module, "AppInterceptor");
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
                    }));
                ctx.routers.push(nidrs::MetaRouter::new(router, meta));
                ctx
            }
            pub async fn get_hello_world2(
                &self,
                meta: Meta,
                Query(q): Query<HashMap<String, String>>,
            ) -> AppResult<(AppendHeaders<[(String, String); 2]>, Json<Status>)> {
                {
                    ::std::io::_print(format_args!("Query {0:?}\n", q));
                };
                {
                    ::std::io::_print(format_args!("Meta Keys {0:?}\n", meta.keys()));
                };
                {
                    ::std::io::_print(format_args!("Meta {0:?}\n", meta.get::<&str>("role")));
                };
                {
                    ::std::io::_print(format_args!("Meta {0:?}\n", meta.get_data::<nidrs::datasets::DisableDefaultPrefix>(),));
                };
                Ok((
                    AppendHeaders([("X-Custom-Header".to_string(), "hello".to_string()), ("X-Custom-Header".to_string(), "world".to_string())]),
                    Json(Status { db: "ok".to_string(), redis: "ok".to_string(), mongo: Mongo { count: 100 } }),
                ))
            }
            pub fn __meta_get_hello_world2(&self) -> nidrs::InnerMeta {
                let mut meta = nidrs::InnerMeta::new();
                meta.set("method_uses", ["AppInterceptor"]);
                meta.set_data(nidrs::openapi::RouterIn(
                    nidrs::openapi::RouterParams::default().comb::<Meta>("meta").comb::<Query<HashMap<String, String>>>(""),
                ));
                meta.set("arr", ["user"]);
                meta.set_data(nidrs::datasets::DisableDefaultPrefix(false));
                meta.set_data(nidrs::datasets::RouterMethod::from("get"));
                meta.set_data(nidrs::openapi::RouterOut(
                    nidrs::openapi::RouterParams::default().comb::<AppResult<(AppendHeaders<[(String, String); 2]>, Json<Status>)>>(""),
                ));
                meta.set_data(nidrs::datasets::RouterPath::from("/hello2"));
                meta.set_data(nidrs::datasets::RouterName::from("get_hello_world2"));
                meta.set("handler", "get_hello_world2");
                meta.set("version", "v2");
                meta.set("service", "AppController");
                meta.set("role", "admin");
                meta.set_data(nidrs::datasets::ServiceName::from("AppController"));
                meta.set_data(nidrs::datasets::ControllerPath::from(""));
                meta.set_data(nidrs::datasets::ServiceType::from("Controller"));
                meta.set("auth", "true");
                meta.set("module", "AppModule");
                meta.set("global", "app");
                meta
            }
            pub fn __route_get_hello_world2(&self, mut ctx: nidrs::ModuleCtx, module: &str) -> nidrs::ModuleCtx {
                use axum::response::IntoResponse;
                use nidrs::externs::axum;
                use nidrs::externs::axum::{extract::Query, Json};
                use nidrs::externs::meta::{InnerMeta, Meta};
                use nidrs::Interceptor;
                use serde_json::Value;
                let mut meta = self.__meta_get_hello_world2();
                let router_info = ctx.get_router_full(&meta);
                if let Err(e) = router_info {
                    {
                        ::core::panicking::panic_fmt(format_args!("[{0}] {1:?}", "__route_get_hello_world2", e));
                    };
                }
                let full_path = router_info.unwrap();
                {
                    ::std::io::_print(format_args!("{0} ", nidrs_extern::colored::Colorize::green("[nidrs]"),));
                };
                {
                    ::std::io::_print(format_args!("Registering router \'{0} {1}\'.\n", "get".to_uppercase(), full_path,));
                };
                meta.set_data(nidrs::datasets::RouterFullPath(full_path.clone()));
                let meta = Meta::new(meta);
                let controller_name = meta.get_data::<nidrs::datasets::ServiceName>().unwrap().value();
                let t_controller = ctx.get_controller::<Self>(module, controller_name);
                let router = nidrs::externs::axum::Router::new()
                    .route(
                        &full_path,
                        nidrs::externs::axum::routing::get(|p0, p1| async move {
                            let r = t_controller.get_hello_world2(p0, p1).await;
                            r
                        }),
                    )
                    .route_layer(nidrs::externs::axum::Extension(meta.clone()))
                    .layer(axum::middleware::from_fn({
                        let inter = ctx.get_interceptor::<AppInterceptor>(module, "AppInterceptor");
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
                    }));
                ctx.routers.push(nidrs::MetaRouter::new(router, meta));
                ctx
            }
            pub async fn post_hello_world(&self, Query(q): Query<HashMap<String, String>>, Json(j): Json<ArgDto>) -> AppResult<String> {
                {
                    ::std::io::_print(format_args!("Query {0:?}\n", q));
                };
                {
                    ::std::io::_print(format_args!("Json {0:?}\n", j));
                };
                Ok("Hello, World2!".to_string())
            }
            pub fn __meta_post_hello_world(&self) -> nidrs::InnerMeta {
                let mut meta = nidrs::InnerMeta::new();
                meta.set_data(nidrs::datasets::RouterMethod::from("post"));
                meta.set_data(nidrs::openapi::RouterOut(nidrs::openapi::RouterParams::default().comb::<AppResult<String>>("")));
                meta.set_data(nidrs::datasets::RouterPath::from("/hello"));
                meta.set_data(nidrs::datasets::RouterName::from("post_hello_world"));
                meta.set_data(nidrs::openapi::RouterIn(
                    nidrs::openapi::RouterParams::default().comb::<Query<HashMap<String, String>>>("").comb::<Json<ArgDto>>(""),
                ));
                meta.set("handler", "post_hello_world");
                meta.set("service", "AppController");
                meta.set("role", "admin");
                meta.set_data(nidrs::datasets::ServiceName::from("AppController"));
                meta.set("version", "v1");
                meta.set_data(nidrs::datasets::ControllerPath::from(""));
                meta.set_data(nidrs::datasets::ServiceType::from("Controller"));
                meta.set("auth", "true");
                meta.set("module", "AppModule");
                meta.set("global", "app");
                meta
            }
            pub fn __route_post_hello_world(&self, mut ctx: nidrs::ModuleCtx, module: &str) -> nidrs::ModuleCtx {
                use axum::response::IntoResponse;
                use nidrs::externs::axum;
                use nidrs::externs::axum::{extract::Query, Json};
                use nidrs::externs::meta::{InnerMeta, Meta};
                use nidrs::Interceptor;
                use serde_json::Value;
                let mut meta = self.__meta_post_hello_world();
                let router_info = ctx.get_router_full(&meta);
                if let Err(e) = router_info {
                    {
                        ::core::panicking::panic_fmt(format_args!("[{0}] {1:?}", "__route_post_hello_world", e));
                    };
                }
                let full_path = router_info.unwrap();
                {
                    ::std::io::_print(format_args!("{0} ", nidrs_extern::colored::Colorize::green("[nidrs]"),));
                };
                {
                    ::std::io::_print(format_args!("Registering router \'{0} {1}\'.\n", "post".to_uppercase(), full_path,));
                };
                meta.set_data(nidrs::datasets::RouterFullPath(full_path.clone()));
                let meta = Meta::new(meta);
                let controller_name = meta.get_data::<nidrs::datasets::ServiceName>().unwrap().value();
                let t_controller = ctx.get_controller::<Self>(module, controller_name);
                let router = nidrs::externs::axum::Router::new()
                    .route(
                        &full_path,
                        nidrs::externs::axum::routing::post(|p0, p1| async move {
                            let r = t_controller.post_hello_world(p0, p1).await;
                            r
                        }),
                    )
                    .route_layer(nidrs::externs::axum::Extension(meta.clone()));
                ctx.routers.push(nidrs::MetaRouter::new(router, meta));
                ctx
            }
        }
    }
    pub mod dto {
        use nidrs::externs::serde_json;
        use nidrs::openapi::utoipa;
        use nidrs::{
            externs::axum::{
                body::Body,
                http::{header, StatusCode},
                response::{IntoResponse, Response},
            },
            valid::dto,
        };
        use utoipa::IntoParams;
        use utoipa::ToSchema;
        pub struct Status {
            pub db: String,
            pub redis: String,
            pub mongo: Mongo,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Status {
                fn serialize<__S>(&self, __serializer: __S) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(__serializer, "Status", false as usize + 1 + 1 + 1)?;
                    _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "db", &self.db)?;
                    _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "redis", &self.redis)?;
                    _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "mongo", &self.mongo)?;
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
                fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(&self, __formatter: &mut _serde::__private::Formatter) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "db" => _serde::__private::Ok(__Field::__field0),
                                "redis" => _serde::__private::Ok(__Field::__field1),
                                "mongo" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"db" => _serde::__private::Ok(__Field::__field0),
                                b"redis" => _serde::__private::Ok(__Field::__field1),
                                b"mongo" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Status>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Status;
                        fn expecting(&self, __formatter: &mut _serde::__private::Formatter) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "struct Status")
                        }
                        #[inline]
                        fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<String>(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct Status with 3 elements"));
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<String>(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct Status with 3 elements"));
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<Mongo>(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(_serde::de::Error::invalid_length(2usize, &"struct Status with 3 elements"));
                                }
                            };
                            _serde::__private::Ok(Status { db: __field0, redis: __field1, mongo: __field2 })
                        }
                        #[inline]
                        fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<Mongo> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("db"));
                                        }
                                        __field0 = _serde::__private::Some(_serde::de::MapAccess::next_value::<String>(&mut __map)?);
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("redis"));
                                        }
                                        __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<String>(&mut __map)?);
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("mongo"));
                                        }
                                        __field2 = _serde::__private::Some(_serde::de::MapAccess::next_value::<Mongo>(&mut __map)?);
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => _serde::__private::de::missing_field("db")?,
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => _serde::__private::de::missing_field("redis")?,
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => _serde::__private::de::missing_field("mongo")?,
                            };
                            _serde::__private::Ok(Status { db: __field0, redis: __field1, mongo: __field2 })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["db", "redis", "mongo"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Status",
                        FIELDS,
                        __Visitor { marker: _serde::__private::PhantomData::<Status>, lifetime: _serde::__private::PhantomData },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for Status {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(f, "Status", "db", &self.db, "redis", &self.redis, "mongo", &&self.mongo)
            }
        }
        impl nidrs::valid::validator::Validator for Status {
            fn valid(&self) -> nidrs::valid::validator::ValidResult {
                use nidrs::valid::ruleset;
                use nidrs::valid::ruleset::*;
                use nidrs::valid::validator::Rule;
                return Ok(());
            }
            fn example(&self) -> Vec<serde_json::Value> {
                ::alloc::vec::Vec::new()
            }
        }
        impl utoipa::IntoParams for Status {
            fn into_params(parameter_in_provider: impl Fn() -> Option<utoipa::openapi::path::ParameterIn>) -> Vec<utoipa::openapi::path::Parameter> {
                [
                    Some(
                        utoipa::openapi::path::ParameterBuilder::new()
                            .name("db")
                            .parameter_in(parameter_in_provider().unwrap_or_default())
                            .required(utoipa::openapi::Required::True)
                            .schema(Some(
                                utoipa::openapi::ObjectBuilder::new()
                                    .schema_type(utoipa::openapi::schema::SchemaType::new(utoipa::openapi::schema::Type::String)),
                            ))
                            .build(),
                    ),
                    Some(
                        utoipa::openapi::path::ParameterBuilder::new()
                            .name("redis")
                            .parameter_in(parameter_in_provider().unwrap_or_default())
                            .required(utoipa::openapi::Required::True)
                            .schema(Some(
                                utoipa::openapi::ObjectBuilder::new()
                                    .schema_type(utoipa::openapi::schema::SchemaType::new(utoipa::openapi::schema::Type::String)),
                            ))
                            .build(),
                    ),
                    Some(
                        utoipa::openapi::path::ParameterBuilder::new()
                            .name("mongo")
                            .parameter_in(parameter_in_provider().unwrap_or_default())
                            .required(utoipa::openapi::Required::True)
                            .schema(Some(utoipa::openapi::schema::RefBuilder::new().ref_location_from_schema_name(::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(format_args!("{0}", <Mongo as utoipa::ToSchema>::name()));
                                res
                            }))))
                            .build(),
                    ),
                ]
                .into_iter()
                .filter(Option::is_some)
                .flatten()
                .collect()
            }
        }
        impl utoipa::__dev::ComposeSchema for Status {
            fn compose(
                mut generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
            ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
                {
                    let mut object = utoipa::openapi::ObjectBuilder::new();
                    object = object
                        .property(
                            "db",
                            utoipa::openapi::ObjectBuilder::new()
                                .schema_type(utoipa::openapi::schema::SchemaType::new(utoipa::openapi::schema::Type::String)),
                        )
                        .required("db");
                    object = object
                        .property(
                            "redis",
                            utoipa::openapi::ObjectBuilder::new()
                                .schema_type(utoipa::openapi::schema::SchemaType::new(utoipa::openapi::schema::Type::String)),
                        )
                        .required("redis");
                    object = object
                        .property(
                            "mongo",
                            utoipa::openapi::schema::RefBuilder::new().ref_location_from_schema_name(::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(format_args!("{0}", <Mongo as utoipa::ToSchema>::name()));
                                res
                            })),
                        )
                        .required("mongo");
                    object
                }
                .into()
            }
        }
        impl utoipa::ToSchema for Status {
            fn name() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed("Status")
            }
            fn schemas(schemas: &mut Vec<(String, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>)>) {
                schemas.extend([(
                    String::from(::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(format_args!("{0}", <Mongo as utoipa::ToSchema>::name()));
                        res
                    })),
                    <Mongo as utoipa::PartialSchema>::schema(),
                )]);
                <Mongo as utoipa::ToSchema>::schemas(schemas);
            }
        }
        impl nidrs::openapi::ToParamDto for Status {
            fn to_param_dto(dto_type: nidrs::openapi::ParamDtoIn) -> nidrs::openapi::ParamDto {
                use nidrs::openapi::utoipa;
                use nidrs::openapi::utoipa::openapi::RefOr;
                use nidrs::openapi::utoipa::openapi::Schema;
                use nidrs::openapi::utoipa::IntoParams;
                use nidrs::openapi::utoipa::ToSchema;
                let ref_schema: RefOr<Schema> = {
                    let mut generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>> = Vec::new();
                    utoipa::openapi::schema::RefBuilder::new().ref_location_from_schema_name(::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(format_args!("{0}", <Self as utoipa::ToSchema>::name()));
                        res
                    }))
                }
                .into();
                let mut schemas: Vec<(String, RefOr<Schema>)> = <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([(
                        <Self as utoipa::ToSchema>::name().to_string(),
                        {
                            let mut generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>> = Vec::new();
                            <Self as utoipa::PartialSchema>::schema()
                        }
                        .into(),
                    )]),
                );
                <Self as utoipa::ToSchema>::schemas(&mut schemas);
                match dto_type {
                    nidrs::openapi::ParamDtoIn::Param(p) => nidrs::openapi::ParamDto::ParamList(Self::into_params(|| Some(p.clone()))),
                    nidrs::openapi::ParamDtoIn::Body => nidrs::openapi::ParamDto::BodySchema((ref_schema, schemas)),
                    _ => nidrs::openapi::ParamDto::None,
                }
            }
        }
        impl IntoResponse for Status {
            fn into_response(self) -> Response {
                let json_body = match serde_json::to_string(&self) {
                    Ok(json) => json,
                    Err(_) => {
                        return Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR).body("Internal server error".into()).unwrap();
                    }
                };
                let res: Response<Body> = Response::builder().header(header::CONTENT_TYPE, "application/json").body(json_body.into()).unwrap();
                res
            }
        }
        pub struct Mongo {
            pub count: u64,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Mongo {
                fn serialize<__S>(&self, __serializer: __S) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(__serializer, "Mongo", false as usize + 1)?;
                    _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "count", &self.count)?;
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
            impl<'de> _serde::Deserialize<'de> for Mongo {
                fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(&self, __formatter: &mut _serde::__private::Formatter) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "count" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"count" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Mongo>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Mongo;
                        fn expecting(&self, __formatter: &mut _serde::__private::Formatter) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "struct Mongo")
                        }
                        #[inline]
                        fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<u64>(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct Mongo with 1 element"));
                                }
                            };
                            _serde::__private::Ok(Mongo { count: __field0 })
                        }
                        #[inline]
                        fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u64> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("count"));
                                        }
                                        __field0 = _serde::__private::Some(_serde::de::MapAccess::next_value::<u64>(&mut __map)?);
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => _serde::__private::de::missing_field("count")?,
                            };
                            _serde::__private::Ok(Mongo { count: __field0 })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["count"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Mongo",
                        FIELDS,
                        __Visitor { marker: _serde::__private::PhantomData::<Mongo>, lifetime: _serde::__private::PhantomData },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for Mongo {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(f, "Mongo", "count", &&self.count)
            }
        }
        impl nidrs::valid::validator::Validator for Mongo {
            fn valid(&self) -> nidrs::valid::validator::ValidResult {
                use nidrs::valid::ruleset;
                use nidrs::valid::ruleset::*;
                use nidrs::valid::validator::Rule;
                return Ok(());
            }
            fn example(&self) -> Vec<serde_json::Value> {
                ::alloc::vec::Vec::new()
            }
        }
        impl utoipa::IntoParams for Mongo {
            fn into_params(parameter_in_provider: impl Fn() -> Option<utoipa::openapi::path::ParameterIn>) -> Vec<utoipa::openapi::path::Parameter> {
                [Some(
                    utoipa::openapi::path::ParameterBuilder::new()
                        .name("count")
                        .parameter_in(parameter_in_provider().unwrap_or_default())
                        .required(utoipa::openapi::Required::True)
                        .schema(Some(
                            utoipa::openapi::ObjectBuilder::new()
                                .schema_type(utoipa::openapi::schema::SchemaType::new(utoipa::openapi::schema::Type::Integer))
                                .format(Some(utoipa::openapi::schema::SchemaFormat::KnownFormat(utoipa::openapi::schema::KnownFormat::Int64)))
                                .minimum(Some(0f64)),
                        ))
                        .build(),
                )]
                .into_iter()
                .filter(Option::is_some)
                .flatten()
                .collect()
            }
        }
        impl utoipa::__dev::ComposeSchema for Mongo {
            fn compose(
                mut generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
            ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
                {
                    let mut object = utoipa::openapi::ObjectBuilder::new();
                    object = object
                        .property(
                            "count",
                            utoipa::openapi::ObjectBuilder::new()
                                .schema_type(utoipa::openapi::schema::SchemaType::new(utoipa::openapi::schema::Type::Integer))
                                .format(Some(utoipa::openapi::schema::SchemaFormat::KnownFormat(utoipa::openapi::schema::KnownFormat::Int64)))
                                .minimum(Some(0f64)),
                        )
                        .required("count");
                    object
                }
                .into()
            }
        }
        impl utoipa::ToSchema for Mongo {
            fn name() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed("Mongo")
            }
            fn schemas(schemas: &mut Vec<(String, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>)>) {
                schemas.extend([]);
            }
        }
        impl nidrs::openapi::ToParamDto for Mongo {
            fn to_param_dto(dto_type: nidrs::openapi::ParamDtoIn) -> nidrs::openapi::ParamDto {
                use nidrs::openapi::utoipa;
                use nidrs::openapi::utoipa::openapi::RefOr;
                use nidrs::openapi::utoipa::openapi::Schema;
                use nidrs::openapi::utoipa::IntoParams;
                use nidrs::openapi::utoipa::ToSchema;
                let ref_schema: RefOr<Schema> = {
                    let mut generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>> = Vec::new();
                    utoipa::openapi::schema::RefBuilder::new().ref_location_from_schema_name(::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(format_args!("{0}", <Self as utoipa::ToSchema>::name()));
                        res
                    }))
                }
                .into();
                let mut schemas: Vec<(String, RefOr<Schema>)> = <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([(
                        <Self as utoipa::ToSchema>::name().to_string(),
                        {
                            let mut generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>> = Vec::new();
                            <Self as utoipa::PartialSchema>::schema()
                        }
                        .into(),
                    )]),
                );
                <Self as utoipa::ToSchema>::schemas(&mut schemas);
                match dto_type {
                    nidrs::openapi::ParamDtoIn::Param(p) => nidrs::openapi::ParamDto::ParamList(Self::into_params(|| Some(p.clone()))),
                    nidrs::openapi::ParamDtoIn::Body => nidrs::openapi::ParamDto::BodySchema((ref_schema, schemas)),
                    _ => nidrs::openapi::ParamDto::None,
                }
            }
        }
        pub struct A {
            pub hello: String,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for A {
                fn serialize<__S>(&self, __serializer: __S) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(__serializer, "A", false as usize + 1)?;
                    _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "hello", &self.hello)?;
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
            impl<'de> _serde::Deserialize<'de> for A {
                fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(&self, __formatter: &mut _serde::__private::Formatter) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "hello" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"hello" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<A>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = A;
                        fn expecting(&self, __formatter: &mut _serde::__private::Formatter) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "struct A")
                        }
                        #[inline]
                        fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<String>(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct A with 1 element"));
                                }
                            };
                            _serde::__private::Ok(A { hello: __field0 })
                        }
                        #[inline]
                        fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("hello"));
                                        }
                                        __field0 = _serde::__private::Some(_serde::de::MapAccess::next_value::<String>(&mut __map)?);
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => _serde::__private::de::missing_field("hello")?,
                            };
                            _serde::__private::Ok(A { hello: __field0 })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["hello"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "A",
                        FIELDS,
                        __Visitor { marker: _serde::__private::PhantomData::<A>, lifetime: _serde::__private::PhantomData },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for A {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(f, "A", "hello", &&self.hello)
            }
        }
        impl nidrs::valid::validator::Validator for A {
            fn valid(&self) -> nidrs::valid::validator::ValidResult {
                use nidrs::valid::ruleset;
                use nidrs::valid::ruleset::*;
                use nidrs::valid::validator::Rule;
                return Ok(());
            }
            fn example(&self) -> Vec<serde_json::Value> {
                ::alloc::vec::Vec::new()
            }
        }
        impl utoipa::__dev::ComposeSchema for A {
            fn compose(
                mut generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
            ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
                {
                    let mut object = utoipa::openapi::ObjectBuilder::new();
                    object = object
                        .property(
                            "hello",
                            utoipa::openapi::ObjectBuilder::new()
                                .schema_type(utoipa::openapi::schema::SchemaType::new(utoipa::openapi::schema::Type::String)),
                        )
                        .required("hello");
                    object
                }
                .into()
            }
        }
        impl utoipa::ToSchema for A {
            fn name() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed("A")
            }
            fn schemas(schemas: &mut Vec<(String, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>)>) {
                schemas.extend([]);
            }
        }
        pub struct B {
            pub hello2: String,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for B {
                fn serialize<__S>(&self, __serializer: __S) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(__serializer, "B", false as usize + 1)?;
                    _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "hello2", &self.hello2)?;
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
            impl<'de> _serde::Deserialize<'de> for B {
                fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(&self, __formatter: &mut _serde::__private::Formatter) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "hello2" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"hello2" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<B>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = B;
                        fn expecting(&self, __formatter: &mut _serde::__private::Formatter) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "struct B")
                        }
                        #[inline]
                        fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<String>(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct B with 1 element"));
                                }
                            };
                            _serde::__private::Ok(B { hello2: __field0 })
                        }
                        #[inline]
                        fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("hello2"));
                                        }
                                        __field0 = _serde::__private::Some(_serde::de::MapAccess::next_value::<String>(&mut __map)?);
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => _serde::__private::de::missing_field("hello2")?,
                            };
                            _serde::__private::Ok(B { hello2: __field0 })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["hello2"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "B",
                        FIELDS,
                        __Visitor { marker: _serde::__private::PhantomData::<B>, lifetime: _serde::__private::PhantomData },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for B {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(f, "B", "hello2", &&self.hello2)
            }
        }
        impl nidrs::valid::validator::Validator for B {
            fn valid(&self) -> nidrs::valid::validator::ValidResult {
                use nidrs::valid::ruleset;
                use nidrs::valid::ruleset::*;
                use nidrs::valid::validator::Rule;
                return Ok(());
            }
            fn example(&self) -> Vec<serde_json::Value> {
                ::alloc::vec::Vec::new()
            }
        }
        impl utoipa::__dev::ComposeSchema for B {
            fn compose(
                mut generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
            ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
                {
                    let mut object = utoipa::openapi::ObjectBuilder::new();
                    object = object
                        .property(
                            "hello2",
                            utoipa::openapi::ObjectBuilder::new()
                                .schema_type(utoipa::openapi::schema::SchemaType::new(utoipa::openapi::schema::Type::String)),
                        )
                        .required("hello2");
                    object
                }
                .into()
            }
        }
        impl utoipa::ToSchema for B {
            fn name() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed("B")
            }
            fn schemas(schemas: &mut Vec<(String, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>)>) {
                schemas.extend([]);
            }
        }
        pub enum ArgDto {
            A(A),
            B(B),
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for ArgDto {
                fn serialize<__S>(&self, __serializer: __S) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    match *self {
                        ArgDto::A(ref __field0) => _serde::Serializer::serialize_newtype_variant(__serializer, "ArgDto", 0u32, "A", __field0),
                        ArgDto::B(ref __field0) => _serde::Serializer::serialize_newtype_variant(__serializer, "ArgDto", 1u32, "B", __field0),
                    }
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for ArgDto {
                fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(&self, __formatter: &mut _serde::__private::Formatter) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "variant identifier")
                        }
                        fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Err(_serde::de::Error::invalid_value(
                                    _serde::de::Unexpected::Unsigned(__value),
                                    &"variant index 0 <= i < 2",
                                )),
                            }
                        }
                        fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "A" => _serde::__private::Ok(__Field::__field0),
                                "B" => _serde::__private::Ok(__Field::__field1),
                                _ => _serde::__private::Err(_serde::de::Error::unknown_variant(__value, VARIANTS)),
                            }
                        }
                        fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"A" => _serde::__private::Ok(__Field::__field0),
                                b"B" => _serde::__private::Ok(__Field::__field1),
                                _ => {
                                    let __value = &_serde::__private::from_utf8_lossy(__value);
                                    _serde::__private::Err(_serde::de::Error::unknown_variant(__value, VARIANTS))
                                }
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<ArgDto>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = ArgDto;
                        fn expecting(&self, __formatter: &mut _serde::__private::Formatter) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "enum ArgDto")
                        }
                        fn visit_enum<__A>(self, __data: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::EnumAccess<'de>,
                        {
                            match _serde::de::EnumAccess::variant(__data)? {
                                (__Field::__field0, __variant) => {
                                    _serde::__private::Result::map(_serde::de::VariantAccess::newtype_variant::<A>(__variant), ArgDto::A)
                                }
                                (__Field::__field1, __variant) => {
                                    _serde::__private::Result::map(_serde::de::VariantAccess::newtype_variant::<B>(__variant), ArgDto::B)
                                }
                            }
                        }
                    }
                    #[doc(hidden)]
                    const VARIANTS: &'static [&'static str] = &["A", "B"];
                    _serde::Deserializer::deserialize_enum(
                        __deserializer,
                        "ArgDto",
                        VARIANTS,
                        __Visitor { marker: _serde::__private::PhantomData::<ArgDto>, lifetime: _serde::__private::PhantomData },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for ArgDto {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    ArgDto::A(__self_0) => ::core::fmt::Formatter::debug_tuple_field1_finish(f, "A", &__self_0),
                    ArgDto::B(__self_0) => ::core::fmt::Formatter::debug_tuple_field1_finish(f, "B", &__self_0),
                }
            }
        }
        impl nidrs::valid::validator::Validator for ArgDto {
            fn valid(&self) -> nidrs::valid::validator::ValidResult {
                use nidrs::valid::ruleset;
                use nidrs::valid::ruleset::*;
                use nidrs::valid::validator::Rule;
                match self {
                    ArgDto::A(v) => v.valid()?,
                    ArgDto::B(v) => v.valid()?,
                }
                return Ok(());
            }
            fn example(&self) -> Vec<serde_json::Value> {
                ::alloc::vec::Vec::new()
            }
        }
        impl utoipa::__dev::ComposeSchema for ArgDto {
            fn compose(
                mut generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
            ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
                Into::<utoipa::openapi::schema::OneOfBuilder>::into(utoipa::openapi::OneOf::with_capacity(2usize))
                    .item(
                        utoipa::openapi::schema::Object::builder()
                            .property(
                                "A",
                                utoipa::openapi::schema::RefBuilder::new().ref_location_from_schema_name(::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(format_args!("{0}", <A as utoipa::ToSchema>::name()));
                                    res
                                })),
                            )
                            .required("A"),
                    )
                    .item(
                        utoipa::openapi::schema::Object::builder()
                            .property(
                                "B",
                                utoipa::openapi::schema::RefBuilder::new().ref_location_from_schema_name(::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(format_args!("{0}", <B as utoipa::ToSchema>::name()));
                                    res
                                })),
                            )
                            .required("B"),
                    )
                    .into()
            }
        }
        impl utoipa::ToSchema for ArgDto {
            fn name() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed("ArgDto")
            }
            fn schemas(schemas: &mut Vec<(String, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>)>) {
                schemas.extend([
                    (
                        String::from(::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(format_args!("{0}", <A as utoipa::ToSchema>::name()));
                            res
                        })),
                        <A as utoipa::PartialSchema>::schema(),
                    ),
                    (
                        String::from(::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(format_args!("{0}", <B as utoipa::ToSchema>::name()));
                            res
                        })),
                        <B as utoipa::PartialSchema>::schema(),
                    ),
                ]);
                <A as utoipa::ToSchema>::schemas(schemas);
                <B as utoipa::ToSchema>::schemas(schemas);
            }
        }
        impl nidrs::openapi::ToParamDto for ArgDto {
            fn to_param_dto(dto_type: nidrs::openapi::ParamDtoIn) -> nidrs::openapi::ParamDto {
                use nidrs::openapi::utoipa;
                use nidrs::openapi::utoipa::openapi::RefOr;
                use nidrs::openapi::utoipa::openapi::Schema;
                use nidrs::openapi::utoipa::ToSchema;
                let ref_schema: RefOr<Schema> = {
                    let mut generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>> = Vec::new();
                    utoipa::openapi::schema::RefBuilder::new().ref_location_from_schema_name(::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(format_args!("{0}", <Self as utoipa::ToSchema>::name()));
                        res
                    }))
                }
                .into();
                let mut schemas: Vec<(String, RefOr<Schema>)> = <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([(
                        <Self as utoipa::ToSchema>::name().to_string(),
                        {
                            let mut generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>> = Vec::new();
                            <Self as utoipa::PartialSchema>::schema()
                        }
                        .into(),
                    )]),
                );
                <Self as utoipa::ToSchema>::schemas(&mut schemas);
                match dto_type {
                    nidrs::openapi::ParamDtoIn::Body => nidrs::openapi::ParamDto::BodySchema((ref_schema, schemas)),
                    _ => nidrs::openapi::ParamDto::None,
                }
            }
        }
        pub struct ArgWrapDto(pub ArgDto);
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for ArgWrapDto {
                fn serialize<__S>(&self, __serializer: __S) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    _serde::Serializer::serialize_newtype_struct(__serializer, "ArgWrapDto", &self.0)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for ArgWrapDto {
                fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<ArgWrapDto>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = ArgWrapDto;
                        fn expecting(&self, __formatter: &mut _serde::__private::Formatter) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "tuple struct ArgWrapDto")
                        }
                        #[inline]
                        fn visit_newtype_struct<__E>(self, __e: __E) -> _serde::__private::Result<Self::Value, __E::Error>
                        where
                            __E: _serde::Deserializer<'de>,
                        {
                            let __field0: ArgDto = <ArgDto as _serde::Deserialize>::deserialize(__e)?;
                            _serde::__private::Ok(ArgWrapDto(__field0))
                        }
                        #[inline]
                        fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<ArgDto>(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(_serde::de::Error::invalid_length(
                                        0usize,
                                        &"tuple struct ArgWrapDto with 1 element",
                                    ));
                                }
                            };
                            _serde::__private::Ok(ArgWrapDto(__field0))
                        }
                    }
                    _serde::Deserializer::deserialize_newtype_struct(
                        __deserializer,
                        "ArgWrapDto",
                        __Visitor { marker: _serde::__private::PhantomData::<ArgWrapDto>, lifetime: _serde::__private::PhantomData },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for ArgWrapDto {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "ArgWrapDto", &&self.0)
            }
        }
        impl nidrs::valid::validator::Validator for ArgWrapDto {
            fn valid(&self) -> nidrs::valid::validator::ValidResult {
                use nidrs::valid::ruleset;
                use nidrs::valid::ruleset::*;
                use nidrs::valid::validator::Rule;
                return Ok(());
            }
            fn example(&self) -> Vec<serde_json::Value> {
                ::alloc::vec::Vec::new()
            }
        }
    }
    pub mod exception {
        pub enum AppException {
            ServiceException(String),
        }
    }
    pub mod interceptor {
        use nidrs::{injectable, AppResult, Interceptor};
        use nidrs_extern::axum::{extract::Request, middleware::Next, response::IntoResponse};
        pub struct AppInterceptor;
        #[automatically_derived]
        impl ::core::default::Default for AppInterceptor {
            #[inline]
            fn default() -> AppInterceptor {
                AppInterceptor {}
            }
        }
        impl nidrs::Service for AppInterceptor {
            fn inject(&self, ctx: nidrs::ModuleCtx, module_name: &str) -> nidrs::ModuleCtx {
                ctx
            }
        }
        impl nidrs::ImplMeta for AppInterceptor {
            fn __meta(&self) -> nidrs::InnerMeta {
                let mut meta = nidrs::InnerMeta::new();
                meta.set_data(nidrs::datasets::ServiceName::from("AppInterceptor"));
                meta.set("service", "AppInterceptor");
                meta.set_data(nidrs::datasets::ServiceType::from("Service"));
                meta.set("module", "AppModule");
                meta.set("global", "app");
                meta
            }
        }
        impl Interceptor for AppInterceptor {
            async fn intercept(&self, req: Request, next: Next) -> AppResult<impl IntoResponse> {
                {
                    ::std::io::_print(format_args!("Intercepting request: {0:?}\n", req));
                };
                let res = next.run(req).await;
                {
                    ::std::io::_print(format_args!("Intercepting response: {0:?}\n", res));
                };
                Ok(res)
            }
        }
    }
    pub mod service {
        use nidrs::{macros::injectable, meta};
        pub struct AppService {}
        #[automatically_derived]
        impl ::core::default::Default for AppService {
            #[inline]
            fn default() -> AppService {
                AppService {}
            }
        }
        impl nidrs::Service for AppService {
            fn inject(&self, ctx: nidrs::ModuleCtx, module_name: &str) -> nidrs::ModuleCtx {
                ctx
            }
        }
        impl nidrs::ImplMeta for AppService {
            fn __meta(&self) -> nidrs::InnerMeta {
                let mut meta = nidrs::InnerMeta::new();
                meta.set_data(nidrs::datasets::ServiceName::from("AppService"));
                meta.set("service", "AppService");
                meta.set("test", true);
                meta.set("test2", true);
                meta.set_data(nidrs::datasets::ServiceType::from("Service"));
                meta.set("module", "AppModule");
                meta.set("global", "app");
                meta
            }
        }
        impl AppService {
            pub fn get_hello_world2(&self) -> String {
                "Hello, nidrs2xx333!".to_string()
            }
        }
    }
    use crate::modules::conf::ConfModule;
    use crate::modules::conf::ConfOptions;
    use crate::modules::user::UserModule;
    use controller::AppController;
    use interceptor::AppInterceptor;
    use service::AppService;
    pub struct AppModule;
    #[automatically_derived]
    impl ::core::default::Default for AppModule {
        #[inline]
        fn default() -> AppModule {
            AppModule {}
        }
    }
    impl nidrs::Module for AppModule {
        fn init(self, mut ctx: nidrs::ModuleCtx) -> nidrs::ModuleCtx {
            use nidrs::{Controller, ImplMeta, InterCtx, Interceptor, InterceptorHandler, ModuleCtx, Service, StateCtx};
            if ctx.modules.contains_key("AppModule") {
                return ctx;
            }
            {
                ::std::io::_print(format_args!("{0} ", nidrs_extern::colored::Colorize::green("[nidrs]"),));
            };
            {
                ::std::io::_print(format_args!("Registering module {0}.\n", "AppModule"));
            };
            ctx.modules.insert("AppModule".to_string(), Box::new(self));
            ctx.imports.insert("AppModule".to_string(), Vec::from(["ConfModule".to_string(), "UserModule".to_string()]));
            ctx.append_exports("AppModule", Vec::<&str>::from(["AppService"]), false);
            ctx.register_interceptor("AppModule", "AppInterceptor", Box::new(std::sync::Arc::new(crate::import::AppInterceptor::default())));
            if ctx.register_controller("AppModule", "AppController", Box::new(std::sync::Arc::new(AppController::default()))) {
                let t_controller = ctx.get_controller::<AppController>("AppModule", "AppController");
                ctx = t_controller.__route_get_hello_world(ctx, "AppModule");
                ctx = t_controller.__route_get_hello_world2(ctx, "AppModule");
                ctx = t_controller.__route_post_hello_world(ctx, "AppModule");
            }
            let svc = std::sync::Arc::new(AppService::default());
            ctx.register_service("AppModule", "AppService", Box::new(svc));
            let mut dyn_module = ConfModule::for_root(ConfOptions { log_level: "info".to_string() });
            let mut dyn_module_wrap = dyn_module.module.take().unwrap();
            let mut dyn_module_services = dyn_module.services;
            dyn_module_services.drain().for_each(|(k, v)| {
                ctx.register_service("ConfModule", &k, v);
            });
            let mut dyn_module_exports = dyn_module.exports;
            ctx.append_exports(
                "ConfModule",
                dyn_module_exports,
                nidrs::get_meta_by_type::<ConfModule>().get_data::<nidrs::datasets::Global>().unwrap_or(&nidrs::datasets::Global(false)).value(),
            );
            let mut ctx = dyn_module_wrap.init(ctx);
            let mut ctx = UserModule::default().init(ctx);
            let t = ctx.get_service::<AppService>("AppModule", "AppService");
            {
                ::std::io::_print(format_args!("{0} ", nidrs_extern::colored::Colorize::green("[nidrs]"),));
            };
            {
                ::std::io::_print(format_args!("Injecting {0}::{1}.\n", "AppModule", "AppService"));
            };
            let ctx = t.inject(ctx, &"AppModule");
            let t = ctx.get_controller::<AppController>("AppModule", "AppController");
            {
                ::std::io::_print(format_args!("{0} ", nidrs_extern::colored::Colorize::green("[nidrs]"),));
            };
            {
                ::std::io::_print(format_args!("Injecting {0}::{1}.\n", "AppModule", "AppController"));
            };
            let ctx = t.inject(ctx, &"AppModule");
            let t = ctx.get_interceptor::<AppInterceptor>("AppModule", "AppInterceptor");
            {
                ::std::io::_print(format_args!("{0} ", nidrs_extern::colored::Colorize::green("[nidrs]"),));
            };
            {
                ::std::io::_print(format_args!("Injecting {0}::{1}.\n", "AppModule", "AppInterceptor"));
            };
            let ctx = t.inject(ctx, &"AppModule");
            ctx
        }
        fn destroy(&self, ctx: &nidrs::ModuleCtx) {
            {
                ::std::io::_print(format_args!("{0} ", nidrs_extern::colored::Colorize::green("[nidrs]"),));
            };
            {
                ::std::io::_print(format_args!("Destroying module {0}.\n", "AppModule"));
            };
        }
    }
    impl nidrs::ImplMeta for AppModule {
        fn __meta(&self) -> nidrs::InnerMeta {
            let mut meta = nidrs::InnerMeta::new();
            meta.set("service", "AppModule");
            meta.set("__", true);
            meta.set("module", "AppModule");
            meta.set("global", "app");
            meta
        }
    }
}
mod modules {
    pub mod conf {
        pub mod options {
            use nidrs::macros::injectable;
            pub struct ConfOptions {
                pub log_level: String,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for ConfOptions {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(f, "ConfOptions", "log_level", &&self.log_level)
                }
            }
            #[automatically_derived]
            impl ::core::default::Default for ConfOptions {
                #[inline]
                fn default() -> ConfOptions {
                    ConfOptions { log_level: ::core::default::Default::default() }
                }
            }
            impl nidrs::Service for ConfOptions {
                fn inject(&self, ctx: nidrs::ModuleCtx, module_name: &str) -> nidrs::ModuleCtx {
                    ctx
                }
            }
            impl nidrs::ImplMeta for ConfOptions {
                fn __meta(&self) -> nidrs::InnerMeta {
                    let mut meta = nidrs::InnerMeta::new();
                    meta.set("service", "ConfOptions");
                    meta.set_data(nidrs::datasets::ServiceType::from("Service"));
                    meta.set_data(nidrs::datasets::ServiceName::from("ConfOptions"));
                    meta.set("module", "ConfModule");
                    meta.set("global", "app");
                    meta
                }
            }
        }
        pub mod service {
            use super::options::ConfOptions;
            use nidrs::macros::{injectable, on_module_init};
            use nidrs::{on_module_destroy, Inject};
            pub struct ConfService {
                pub options: Inject<ConfOptions>,
                pub log_level: String,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for ConfService {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(f, "ConfService", "options", &self.options, "log_level", &&self.log_level)
                }
            }
            #[automatically_derived]
            impl ::core::default::Default for ConfService {
                #[inline]
                fn default() -> ConfService {
                    ConfService { options: ::core::default::Default::default(), log_level: ::core::default::Default::default() }
                }
            }
            impl nidrs::Service for ConfService {
                fn inject(&self, ctx: nidrs::ModuleCtx, module_name: &str) -> nidrs::ModuleCtx {
                    let service = ctx.get_service::<ConfOptions>(&module_name, "ConfOptions");
                    self.options.inject(service.clone());
                    ctx
                }
            }
            impl nidrs::ImplMeta for ConfService {
                fn __meta(&self) -> nidrs::InnerMeta {
                    let mut meta = nidrs::InnerMeta::new();
                    meta.set_data(nidrs::datasets::ServiceType::from("Service"));
                    meta.set_data(nidrs::datasets::ServiceName::from("ConfService"));
                    meta.set("service", "ConfService");
                    meta.set("module", "ConfModule");
                    meta.set("global", "app");
                    meta
                }
            }
            impl ConfService {
                pub fn on_module_init(&self) {
                    let options = self.options.extract();
                    {
                        ::std::io::_print(format_args!("ConfService initialized with log_level: {0:?}\n", options,));
                    };
                }
                pub fn on_module_destroy(&self) {
                    {
                        ::std::io::_print(format_args!("ConfService destroyed\n"));
                    };
                }
            }
        }
        use nidrs::macros::module;
        use nidrs::{DynamicModule, Service};
        pub use options::ConfOptions;
        use service::ConfService;
        pub struct ConfModule;
        #[automatically_derived]
        impl ::core::default::Default for ConfModule {
            #[inline]
            fn default() -> ConfModule {
                ConfModule {}
            }
        }
        impl nidrs::Module for ConfModule {
            fn init(self, mut ctx: nidrs::ModuleCtx) -> nidrs::ModuleCtx {
                use nidrs::{Controller, ImplMeta, InterCtx, Interceptor, InterceptorHandler, ModuleCtx, Service, StateCtx};
                if ctx.modules.contains_key("ConfModule") {
                    return ctx;
                }
                {
                    ::std::io::_print(format_args!("{0} ", nidrs_extern::colored::Colorize::green("[nidrs]"),));
                };
                {
                    ::std::io::_print(format_args!("Registering module {0}.\n", "ConfModule"));
                };
                ctx.modules.insert("ConfModule".to_string(), Box::new(self));
                ctx.imports.insert("ConfModule".to_string(), Vec::from([]));
                ctx.append_exports("ConfModule", Vec::<&str>::from(["ConfService"]), false);
                let svc = std::sync::Arc::new(ConfService::default());
                ctx.register_service("ConfModule", "ConfService", Box::new(svc));
                let t = ctx.get_service::<ConfService>("ConfModule", "ConfService");
                {
                    ::std::io::_print(format_args!("{0} ", nidrs_extern::colored::Colorize::green("[nidrs]"),));
                };
                {
                    ::std::io::_print(format_args!("Injecting {0}::{1}.\n", "ConfModule", "ConfService",));
                };
                let ctx = t.inject(ctx, &"ConfModule");
                let service = ctx.get_service::<ConfService>("ConfModule", "ConfService");
                {
                    ::std::io::_print(format_args!("{0} ", nidrs_extern::colored::Colorize::green("[nidrs]"),));
                };
                {
                    ::std::io::_print(format_args!("Triggering event {0} for {1}::{2}.\n", "on_module_init", "ConfModule", "ConfService",));
                };
                service.on_module_init();
                ctx
            }
            fn destroy(&self, ctx: &nidrs::ModuleCtx) {
                let service = ctx.get_service::<ConfService>("ConfModule", "ConfService");
                {
                    ::std::io::_print(format_args!("{0} ", nidrs_extern::colored::Colorize::green("[nidrs]"),));
                };
                {
                    ::std::io::_print(format_args!("Triggering event {0} for {1}::{2}.\n", "on_module_destroy", "ConfModule", "ConfService",));
                };
                service.on_module_destroy();
                {
                    ::std::io::_print(format_args!("{0} ", nidrs_extern::colored::Colorize::green("[nidrs]"),));
                };
                {
                    ::std::io::_print(format_args!("Destroying module {0}.\n", "ConfModule"));
                };
            }
        }
        impl nidrs::ImplMeta for ConfModule {
            fn __meta(&self) -> nidrs::InnerMeta {
                let mut meta = nidrs::InnerMeta::new();
                meta.set("__", true);
                meta.set("service", "ConfModule");
                meta.set("module", "ConfModule");
                meta.set("global", "app");
                meta
            }
        }
        impl ConfModule {
            pub fn for_root(options: ConfOptions) -> DynamicModule<Self> {
                DynamicModule::new(Self).service(options)
            }
        }
    }
    pub mod user {
        use nidrs::macros::module;
        pub mod controller {
            use super::dto::{CreateUserResDto, FilterDto};
            use super::{dto::CreateUserDto, dto::UserByIdDto, service::UserService};
            use nidrs::macros::{controller, get, meta};
            use nidrs::openapi::api;
            use nidrs::{externs::axum::extract::Query, post};
            use nidrs::{AppResult, Inject};
            use nidrs_extern::axum::extract::Path;
            use nidrs_extern::axum::Json;
            use std::collections::HashMap;
            pub struct UserController {
                user_service: Inject<UserService>,
            }
            #[automatically_derived]
            impl ::core::default::Default for UserController {
                #[inline]
                fn default() -> UserController {
                    UserController { user_service: ::core::default::Default::default() }
                }
            }
            impl nidrs::Controller for UserController {}
            impl nidrs::Service for UserController {
                fn inject(&self, ctx: nidrs::ModuleCtx, module_name: &str) -> nidrs::ModuleCtx {
                    let service = ctx.get_service::<UserService>(&module_name, "UserService");
                    self.user_service.inject(service.clone());
                    ctx
                }
            }
            impl nidrs::ImplMeta for UserController {
                fn __meta(&self) -> nidrs::InnerMeta {
                    let mut meta = nidrs::InnerMeta::new();
                    meta.set_data(nidrs::datasets::ServiceType::from("Controller"));
                    meta.set_data(nidrs::datasets::ControllerPath::from("/user"));
                    meta.set("service", "UserController");
                    meta.set_data(nidrs::datasets::ServiceName::from("UserController"));
                    meta.set("module", "UserModule");
                    meta.set("global", "app");
                    meta
                }
            }
            impl UserController {
                pub async fn get_all(&self, Query(q): Query<HashMap<String, String>>) -> AppResult<String> {
                    {
                        ::std::io::_print(format_args!("Query {0:?}\n", q));
                    };
                    Ok(self.user_service.extract().get_hello_world2())
                }
                pub fn __meta_get_all(&self) -> nidrs::InnerMeta {
                    let mut meta = nidrs::InnerMeta::new();
                    meta.set_data(nidrs::datasets::RouterPath::from("/"));
                    meta.set_data(nidrs::openapi::RouterOut(nidrs::openapi::RouterParams::default().comb::<AppResult<String>>("")));
                    meta.set_data(nidrs::openapi::RouterIn(nidrs::openapi::RouterParams::default().comb::<Query<HashMap<String, String>>>("")));
                    meta.set("handler", "get_all");
                    meta.set_data(nidrs::datasets::RouterMethod::from("get"));
                    meta.set_data(nidrs::datasets::RouterName::from("get_all"));
                    meta.set_data(nidrs::datasets::ServiceType::from("Controller"));
                    meta.set_data(nidrs::datasets::ControllerPath::from("/user"));
                    meta.set("service", "UserController");
                    meta.set_data(nidrs::datasets::ServiceName::from("UserController"));
                    meta.set("module", "UserModule");
                    meta.set("global", "app");
                    meta
                }
                pub fn __route_get_all(&self, mut ctx: nidrs::ModuleCtx, module: &str) -> nidrs::ModuleCtx {
                    use axum::response::IntoResponse;
                    use nidrs::externs::axum;
                    use nidrs::externs::axum::{extract::Query, Json};
                    use nidrs::externs::meta::{InnerMeta, Meta};
                    use nidrs::Interceptor;
                    use serde_json::Value;
                    let mut meta = self.__meta_get_all();
                    let router_info = ctx.get_router_full(&meta);
                    if let Err(e) = router_info {
                        {
                            ::core::panicking::panic_fmt(format_args!("[{0}] {1:?}", "__route_get_all", e));
                        };
                    }
                    let full_path = router_info.unwrap();
                    {
                        ::std::io::_print(format_args!("{0} ", nidrs_extern::colored::Colorize::green("[nidrs]"),));
                    };
                    {
                        ::std::io::_print(format_args!("Registering router \'{0} {1}\'.\n", "get".to_uppercase(), full_path,));
                    };
                    meta.set_data(nidrs::datasets::RouterFullPath(full_path.clone()));
                    let meta = Meta::new(meta);
                    let controller_name = meta.get_data::<nidrs::datasets::ServiceName>().unwrap().value();
                    let t_controller = ctx.get_controller::<Self>(module, controller_name);
                    let router = nidrs::externs::axum::Router::new()
                        .route(
                            &full_path,
                            nidrs::externs::axum::routing::get(|p0| async move {
                                let r = t_controller.get_all(p0).await;
                                r
                            }),
                        )
                        .route_layer(nidrs::externs::axum::Extension(meta.clone()));
                    ctx.routers.push(nidrs::MetaRouter::new(router, meta));
                    ctx
                }
                pub async fn get_one(&self, id: Path<UserByIdDto>, query: Query<FilterDto>) -> AppResult<String> {
                    Ok(::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(format_args!("get one! id: {0}", id.id));
                        res
                    }))
                }
                pub fn __meta_get_one(&self) -> nidrs::InnerMeta {
                    let mut meta = nidrs::InnerMeta::new();
                    meta.set_data(nidrs::openapi::RouterOut(nidrs::openapi::RouterParams::default().comb::<AppResult<String>>("")));
                    meta.set_data(nidrs::datasets::RouterMethod::from("get"));
                    meta.set("handler", "get_one");
                    meta.set_data(nidrs::openapi::RouterIn(
                        nidrs::openapi::RouterParams::default().comb::<Path<UserByIdDto>>("id").comb::<Query<FilterDto>>("query"),
                    ));
                    meta.set_data(nidrs::datasets::RouterName::from("get_one"));
                    meta.set_data(nidrs::datasets::RouterPath::from("/:id"));
                    meta.set_data(nidrs::datasets::ServiceType::from("Controller"));
                    meta.set_data(nidrs::datasets::ControllerPath::from("/user"));
                    meta.set("service", "UserController");
                    meta.set_data(nidrs::datasets::ServiceName::from("UserController"));
                    meta.set("module", "UserModule");
                    meta.set("global", "app");
                    meta
                }
                pub fn __route_get_one(&self, mut ctx: nidrs::ModuleCtx, module: &str) -> nidrs::ModuleCtx {
                    use axum::response::IntoResponse;
                    use nidrs::externs::axum;
                    use nidrs::externs::axum::{extract::Query, Json};
                    use nidrs::externs::meta::{InnerMeta, Meta};
                    use nidrs::Interceptor;
                    use serde_json::Value;
                    let mut meta = self.__meta_get_one();
                    let router_info = ctx.get_router_full(&meta);
                    if let Err(e) = router_info {
                        {
                            ::core::panicking::panic_fmt(format_args!("[{0}] {1:?}", "__route_get_one", e));
                        };
                    }
                    let full_path = router_info.unwrap();
                    {
                        ::std::io::_print(format_args!("{0} ", nidrs_extern::colored::Colorize::green("[nidrs]"),));
                    };
                    {
                        ::std::io::_print(format_args!("Registering router \'{0} {1}\'.\n", "get".to_uppercase(), full_path,));
                    };
                    meta.set_data(nidrs::datasets::RouterFullPath(full_path.clone()));
                    let meta = Meta::new(meta);
                    let controller_name = meta.get_data::<nidrs::datasets::ServiceName>().unwrap().value();
                    let t_controller = ctx.get_controller::<Self>(module, controller_name);
                    let router = nidrs::externs::axum::Router::new()
                        .route(
                            &full_path,
                            nidrs::externs::axum::routing::get(|p0, p1| async move {
                                let r = t_controller.get_one(p0, p1).await;
                                r
                            }),
                        )
                        .route_layer(nidrs::externs::axum::Extension(meta.clone()));
                    ctx.routers.push(nidrs::MetaRouter::new(router, meta));
                    ctx
                }
                pub async fn create_user(&self, dto: Json<CreateUserDto>) -> AppResult<Json<CreateUserResDto>> {
                    Ok(Json(CreateUserResDto { id: 1, name: dto.name.clone() }))
                }
                pub fn __meta_create_user(&self) -> nidrs::InnerMeta {
                    let mut meta = nidrs::InnerMeta::new();
                    meta.set_data(nidrs::openapi::RouterOut(nidrs::openapi::RouterParams::default().comb::<AppResult<Json<CreateUserResDto>>>("")));
                    meta.set_data(nidrs::openapi::RouterIn(nidrs::openapi::RouterParams::default().comb::<Json<CreateUserDto>>("dto")));
                    meta.set_data(nidrs::datasets::RouterPath::from("/"));
                    meta.set_data(nidrs::datasets::RouterName::from("create_user"));
                    meta.set("handler", "create_user");
                    meta.set_data(nidrs::datasets::RouterMethod::from("post"));
                    meta.set_data(nidrs::datasets::ServiceType::from("Controller"));
                    meta.set_data(nidrs::datasets::ControllerPath::from("/user"));
                    meta.set("service", "UserController");
                    meta.set_data(nidrs::datasets::ServiceName::from("UserController"));
                    meta.set("module", "UserModule");
                    meta.set("global", "app");
                    meta
                }
                pub fn __route_create_user(&self, mut ctx: nidrs::ModuleCtx, module: &str) -> nidrs::ModuleCtx {
                    use axum::response::IntoResponse;
                    use nidrs::externs::axum;
                    use nidrs::externs::axum::{extract::Query, Json};
                    use nidrs::externs::meta::{InnerMeta, Meta};
                    use nidrs::Interceptor;
                    use serde_json::Value;
                    let mut meta = self.__meta_create_user();
                    let router_info = ctx.get_router_full(&meta);
                    if let Err(e) = router_info {
                        {
                            ::core::panicking::panic_fmt(format_args!("[{0}] {1:?}", "__route_create_user", e));
                        };
                    }
                    let full_path = router_info.unwrap();
                    {
                        ::std::io::_print(format_args!("{0} ", nidrs_extern::colored::Colorize::green("[nidrs]"),));
                    };
                    {
                        ::std::io::_print(format_args!("Registering router \'{0} {1}\'.\n", "post".to_uppercase(), full_path,));
                    };
                    meta.set_data(nidrs::datasets::RouterFullPath(full_path.clone()));
                    let meta = Meta::new(meta);
                    let controller_name = meta.get_data::<nidrs::datasets::ServiceName>().unwrap().value();
                    let t_controller = ctx.get_controller::<Self>(module, controller_name);
                    let router = nidrs::externs::axum::Router::new()
                        .route(
                            &full_path,
                            nidrs::externs::axum::routing::post(|p0| async move {
                                let r = t_controller.create_user(p0).await;
                                r
                            }),
                        )
                        .route_layer(nidrs::externs::axum::Extension(meta.clone()));
                    ctx.routers.push(nidrs::MetaRouter::new(router, meta));
                    ctx
                }
            }
        }
        pub mod dto {
            use nidrs::openapi::utoipa;
            pub struct CreateUserDto {
                pub name: String,
                pub age: i32,
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for CreateUserDto {
                    fn serialize<__S>(&self, __serializer: __S) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(__serializer, "CreateUserDto", false as usize + 1 + 1)?;
                        _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "name", &self.name)?;
                        _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "age", &self.age)?;
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
                impl<'de> _serde::Deserialize<'de> for CreateUserDto {
                    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
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
                            fn expecting(&self, __formatter: &mut _serde::__private::Formatter) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(__formatter, "field identifier")
                            }
                            fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "name" => _serde::__private::Ok(__Field::__field0),
                                    "age" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"name" => _serde::__private::Ok(__Field::__field0),
                                    b"age" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<CreateUserDto>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = CreateUserDto;
                            fn expecting(&self, __formatter: &mut _serde::__private::Formatter) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(__formatter, "struct CreateUserDto")
                            }
                            #[inline]
                            fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<String>(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(_serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct CreateUserDto with 2 elements",
                                        ));
                                    }
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<i32>(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(_serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct CreateUserDto with 2 elements",
                                        ));
                                    }
                                };
                                _serde::__private::Ok(CreateUserDto { name: __field0, age: __field1 })
                            }
                            #[inline]
                            fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                                let mut __field1: _serde::__private::Option<i32> = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("name"));
                                            }
                                            __field0 = _serde::__private::Some(_serde::de::MapAccess::next_value::<String>(&mut __map)?);
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("age"));
                                            }
                                            __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<i32>(&mut __map)?);
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => _serde::__private::de::missing_field("name")?,
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => _serde::__private::de::missing_field("age")?,
                                };
                                _serde::__private::Ok(CreateUserDto { name: __field0, age: __field1 })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["name", "age"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "CreateUserDto",
                            FIELDS,
                            __Visitor { marker: _serde::__private::PhantomData::<CreateUserDto>, lifetime: _serde::__private::PhantomData },
                        )
                    }
                }
            };
            #[automatically_derived]
            impl ::core::fmt::Debug for CreateUserDto {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(f, "CreateUserDto", "name", &self.name, "age", &&self.age)
                }
            }
            impl nidrs::valid::validator::Validator for CreateUserDto {
                fn valid(&self) -> nidrs::valid::validator::ValidResult {
                    use nidrs::valid::ruleset;
                    use nidrs::valid::ruleset::*;
                    use nidrs::valid::validator::Rule;
                    return Ok(());
                }
                fn example(&self) -> Vec<serde_json::Value> {
                    ::alloc::vec::Vec::new()
                }
            }
            impl utoipa::IntoParams for CreateUserDto {
                fn into_params(
                    parameter_in_provider: impl Fn() -> Option<utoipa::openapi::path::ParameterIn>,
                ) -> Vec<utoipa::openapi::path::Parameter> {
                    [
                        Some(
                            utoipa::openapi::path::ParameterBuilder::new()
                                .name("name")
                                .parameter_in(parameter_in_provider().unwrap_or_default())
                                .required(utoipa::openapi::Required::True)
                                .schema(Some(
                                    utoipa::openapi::ObjectBuilder::new()
                                        .schema_type(utoipa::openapi::schema::SchemaType::new(utoipa::openapi::schema::Type::String)),
                                ))
                                .build(),
                        ),
                        Some(
                            utoipa::openapi::path::ParameterBuilder::new()
                                .name("age")
                                .parameter_in(parameter_in_provider().unwrap_or_default())
                                .required(utoipa::openapi::Required::True)
                                .schema(Some(
                                    utoipa::openapi::ObjectBuilder::new()
                                        .schema_type(utoipa::openapi::schema::SchemaType::new(utoipa::openapi::schema::Type::Integer))
                                        .format(Some(utoipa::openapi::schema::SchemaFormat::KnownFormat(
                                            utoipa::openapi::schema::KnownFormat::Int32,
                                        ))),
                                ))
                                .build(),
                        ),
                    ]
                    .into_iter()
                    .filter(Option::is_some)
                    .flatten()
                    .collect()
                }
            }
            impl utoipa::__dev::ComposeSchema for CreateUserDto {
                fn compose(
                    mut generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
                ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
                    {
                        let mut object = utoipa::openapi::ObjectBuilder::new();
                        object = object
                            .property(
                                "name",
                                utoipa::openapi::ObjectBuilder::new()
                                    .schema_type(utoipa::openapi::schema::SchemaType::new(utoipa::openapi::schema::Type::String)),
                            )
                            .required("name");
                        object = object
                            .property(
                                "age",
                                utoipa::openapi::ObjectBuilder::new()
                                    .schema_type(utoipa::openapi::schema::SchemaType::new(utoipa::openapi::schema::Type::Integer))
                                    .format(Some(utoipa::openapi::schema::SchemaFormat::KnownFormat(utoipa::openapi::schema::KnownFormat::Int32))),
                            )
                            .required("age");
                        object
                    }
                    .into()
                }
            }
            impl utoipa::ToSchema for CreateUserDto {
                fn name() -> std::borrow::Cow<'static, str> {
                    std::borrow::Cow::Borrowed("CreateUserDto")
                }
                fn schemas(schemas: &mut Vec<(String, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>)>) {
                    schemas.extend([]);
                }
            }
            impl nidrs::openapi::ToParamDto for CreateUserDto {
                fn to_param_dto(dto_type: nidrs::openapi::ParamDtoIn) -> nidrs::openapi::ParamDto {
                    use nidrs::openapi::utoipa;
                    use nidrs::openapi::utoipa::openapi::RefOr;
                    use nidrs::openapi::utoipa::openapi::Schema;
                    use nidrs::openapi::utoipa::IntoParams;
                    use nidrs::openapi::utoipa::ToSchema;
                    let ref_schema: RefOr<Schema> = {
                        let mut generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>> = Vec::new();
                        utoipa::openapi::schema::RefBuilder::new().ref_location_from_schema_name(::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(format_args!("{0}", <Self as utoipa::ToSchema>::name()));
                            res
                        }))
                    }
                    .into();
                    let mut schemas: Vec<(String, RefOr<Schema>)> = <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([(
                            <Self as utoipa::ToSchema>::name().to_string(),
                            {
                                let mut generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>> = Vec::new();
                                <Self as utoipa::PartialSchema>::schema()
                            }
                            .into(),
                        )]),
                    );
                    <Self as utoipa::ToSchema>::schemas(&mut schemas);
                    match dto_type {
                        nidrs::openapi::ParamDtoIn::Param(p) => nidrs::openapi::ParamDto::ParamList(Self::into_params(|| Some(p.clone()))),
                        nidrs::openapi::ParamDtoIn::Body => nidrs::openapi::ParamDto::BodySchema((ref_schema, schemas)),
                        _ => nidrs::openapi::ParamDto::None,
                    }
                }
            }
            pub struct UserByIdDto {
                pub id: i32,
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for UserByIdDto {
                    fn serialize<__S>(&self, __serializer: __S) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(__serializer, "UserByIdDto", false as usize + 1)?;
                        _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "id", &self.id)?;
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
                impl<'de> _serde::Deserialize<'de> for UserByIdDto {
                    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(&self, __formatter: &mut _serde::__private::Formatter) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(__formatter, "field identifier")
                            }
                            fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "id" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"id" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<UserByIdDto>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = UserByIdDto;
                            fn expecting(&self, __formatter: &mut _serde::__private::Formatter) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(__formatter, "struct UserByIdDto")
                            }
                            #[inline]
                            fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<i32>(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(_serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct UserByIdDto with 1 element",
                                        ));
                                    }
                                };
                                _serde::__private::Ok(UserByIdDto { id: __field0 })
                            }
                            #[inline]
                            fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<i32> = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("id"));
                                            }
                                            __field0 = _serde::__private::Some(_serde::de::MapAccess::next_value::<i32>(&mut __map)?);
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => _serde::__private::de::missing_field("id")?,
                                };
                                _serde::__private::Ok(UserByIdDto { id: __field0 })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["id"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "UserByIdDto",
                            FIELDS,
                            __Visitor { marker: _serde::__private::PhantomData::<UserByIdDto>, lifetime: _serde::__private::PhantomData },
                        )
                    }
                }
            };
            #[automatically_derived]
            impl ::core::fmt::Debug for UserByIdDto {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(f, "UserByIdDto", "id", &&self.id)
                }
            }
            impl nidrs::valid::validator::Validator for UserByIdDto {
                fn valid(&self) -> nidrs::valid::validator::ValidResult {
                    use nidrs::valid::ruleset;
                    use nidrs::valid::ruleset::*;
                    use nidrs::valid::validator::Rule;
                    return Ok(());
                }
                fn example(&self) -> Vec<serde_json::Value> {
                    ::alloc::vec::Vec::new()
                }
            }
            impl utoipa::IntoParams for UserByIdDto {
                fn into_params(
                    parameter_in_provider: impl Fn() -> Option<utoipa::openapi::path::ParameterIn>,
                ) -> Vec<utoipa::openapi::path::Parameter> {
                    [Some(
                        utoipa::openapi::path::ParameterBuilder::new()
                            .name("id")
                            .parameter_in(parameter_in_provider().unwrap_or_default())
                            .required(utoipa::openapi::Required::True)
                            .schema(Some(
                                utoipa::openapi::ObjectBuilder::new()
                                    .schema_type(utoipa::openapi::schema::SchemaType::new(utoipa::openapi::schema::Type::Integer))
                                    .format(Some(utoipa::openapi::schema::SchemaFormat::KnownFormat(utoipa::openapi::schema::KnownFormat::Int32))),
                            ))
                            .build(),
                    )]
                    .into_iter()
                    .filter(Option::is_some)
                    .flatten()
                    .collect()
                }
            }
            impl utoipa::__dev::ComposeSchema for UserByIdDto {
                fn compose(
                    mut generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
                ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
                    {
                        let mut object = utoipa::openapi::ObjectBuilder::new();
                        object = object
                            .property(
                                "id",
                                utoipa::openapi::ObjectBuilder::new()
                                    .schema_type(utoipa::openapi::schema::SchemaType::new(utoipa::openapi::schema::Type::Integer))
                                    .format(Some(utoipa::openapi::schema::SchemaFormat::KnownFormat(utoipa::openapi::schema::KnownFormat::Int32))),
                            )
                            .required("id");
                        object
                    }
                    .into()
                }
            }
            impl utoipa::ToSchema for UserByIdDto {
                fn name() -> std::borrow::Cow<'static, str> {
                    std::borrow::Cow::Borrowed("UserByIdDto")
                }
                fn schemas(schemas: &mut Vec<(String, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>)>) {
                    schemas.extend([]);
                }
            }
            impl nidrs::openapi::ToParamDto for UserByIdDto {
                fn to_param_dto(dto_type: nidrs::openapi::ParamDtoIn) -> nidrs::openapi::ParamDto {
                    use nidrs::openapi::utoipa;
                    use nidrs::openapi::utoipa::openapi::RefOr;
                    use nidrs::openapi::utoipa::openapi::Schema;
                    use nidrs::openapi::utoipa::IntoParams;
                    use nidrs::openapi::utoipa::ToSchema;
                    let ref_schema: RefOr<Schema> = {
                        let mut generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>> = Vec::new();
                        utoipa::openapi::schema::RefBuilder::new().ref_location_from_schema_name(::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(format_args!("{0}", <Self as utoipa::ToSchema>::name()));
                            res
                        }))
                    }
                    .into();
                    let mut schemas: Vec<(String, RefOr<Schema>)> = <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([(
                            <Self as utoipa::ToSchema>::name().to_string(),
                            {
                                let mut generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>> = Vec::new();
                                <Self as utoipa::PartialSchema>::schema()
                            }
                            .into(),
                        )]),
                    );
                    <Self as utoipa::ToSchema>::schemas(&mut schemas);
                    match dto_type {
                        nidrs::openapi::ParamDtoIn::Param(p) => nidrs::openapi::ParamDto::ParamList(Self::into_params(|| Some(p.clone()))),
                        nidrs::openapi::ParamDtoIn::Body => nidrs::openapi::ParamDto::BodySchema((ref_schema, schemas)),
                        _ => nidrs::openapi::ParamDto::None,
                    }
                }
            }
            pub struct FilterDto {
                pub id: i32,
                pub filter: String,
                pub page: i32,
                pub size: i32,
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for FilterDto {
                    fn serialize<__S>(&self, __serializer: __S) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(__serializer, "FilterDto", false as usize + 1 + 1 + 1 + 1)?;
                        _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "id", &self.id)?;
                        _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "filter", &self.filter)?;
                        _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "page", &self.page)?;
                        _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "size", &self.size)?;
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
                impl<'de> _serde::Deserialize<'de> for FilterDto {
                    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __field1,
                            __field2,
                            __field3,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(&self, __formatter: &mut _serde::__private::Formatter) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(__formatter, "field identifier")
                            }
                            fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    2u64 => _serde::__private::Ok(__Field::__field2),
                                    3u64 => _serde::__private::Ok(__Field::__field3),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "id" => _serde::__private::Ok(__Field::__field0),
                                    "filter" => _serde::__private::Ok(__Field::__field1),
                                    "page" => _serde::__private::Ok(__Field::__field2),
                                    "size" => _serde::__private::Ok(__Field::__field3),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"id" => _serde::__private::Ok(__Field::__field0),
                                    b"filter" => _serde::__private::Ok(__Field::__field1),
                                    b"page" => _serde::__private::Ok(__Field::__field2),
                                    b"size" => _serde::__private::Ok(__Field::__field3),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<FilterDto>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = FilterDto;
                            fn expecting(&self, __formatter: &mut _serde::__private::Formatter) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(__formatter, "struct FilterDto")
                            }
                            #[inline]
                            fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<i32>(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(_serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct FilterDto with 4 elements",
                                        ));
                                    }
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<String>(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(_serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct FilterDto with 4 elements",
                                        ));
                                    }
                                };
                                let __field2 = match _serde::de::SeqAccess::next_element::<i32>(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(_serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct FilterDto with 4 elements",
                                        ));
                                    }
                                };
                                let __field3 = match _serde::de::SeqAccess::next_element::<i32>(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(_serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct FilterDto with 4 elements",
                                        ));
                                    }
                                };
                                _serde::__private::Ok(FilterDto { id: __field0, filter: __field1, page: __field2, size: __field3 })
                            }
                            #[inline]
                            fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<i32> = _serde::__private::None;
                                let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                                let mut __field2: _serde::__private::Option<i32> = _serde::__private::None;
                                let mut __field3: _serde::__private::Option<i32> = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("id"));
                                            }
                                            __field0 = _serde::__private::Some(_serde::de::MapAccess::next_value::<i32>(&mut __map)?);
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("filter"));
                                            }
                                            __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<String>(&mut __map)?);
                                        }
                                        __Field::__field2 => {
                                            if _serde::__private::Option::is_some(&__field2) {
                                                return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("page"));
                                            }
                                            __field2 = _serde::__private::Some(_serde::de::MapAccess::next_value::<i32>(&mut __map)?);
                                        }
                                        __Field::__field3 => {
                                            if _serde::__private::Option::is_some(&__field3) {
                                                return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("size"));
                                            }
                                            __field3 = _serde::__private::Some(_serde::de::MapAccess::next_value::<i32>(&mut __map)?);
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => _serde::__private::de::missing_field("id")?,
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => _serde::__private::de::missing_field("filter")?,
                                };
                                let __field2 = match __field2 {
                                    _serde::__private::Some(__field2) => __field2,
                                    _serde::__private::None => _serde::__private::de::missing_field("page")?,
                                };
                                let __field3 = match __field3 {
                                    _serde::__private::Some(__field3) => __field3,
                                    _serde::__private::None => _serde::__private::de::missing_field("size")?,
                                };
                                _serde::__private::Ok(FilterDto { id: __field0, filter: __field1, page: __field2, size: __field3 })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["id", "filter", "page", "size"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "FilterDto",
                            FIELDS,
                            __Visitor { marker: _serde::__private::PhantomData::<FilterDto>, lifetime: _serde::__private::PhantomData },
                        )
                    }
                }
            };
            #[automatically_derived]
            impl ::core::fmt::Debug for FilterDto {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field4_finish(
                        f,
                        "FilterDto",
                        "id",
                        &self.id,
                        "filter",
                        &self.filter,
                        "page",
                        &self.page,
                        "size",
                        &&self.size,
                    )
                }
            }
            impl nidrs::valid::validator::Validator for FilterDto {
                fn valid(&self) -> nidrs::valid::validator::ValidResult {
                    use nidrs::valid::ruleset;
                    use nidrs::valid::ruleset::*;
                    use nidrs::valid::validator::Rule;
                    return Ok(());
                }
                fn example(&self) -> Vec<serde_json::Value> {
                    ::alloc::vec::Vec::new()
                }
            }
            impl utoipa::IntoParams for FilterDto {
                fn into_params(
                    parameter_in_provider: impl Fn() -> Option<utoipa::openapi::path::ParameterIn>,
                ) -> Vec<utoipa::openapi::path::Parameter> {
                    [
                        Some(
                            utoipa::openapi::path::ParameterBuilder::new()
                                .name("id")
                                .parameter_in(parameter_in_provider().unwrap_or_default())
                                .required(utoipa::openapi::Required::True)
                                .schema(Some(
                                    utoipa::openapi::ObjectBuilder::new()
                                        .schema_type(utoipa::openapi::schema::SchemaType::new(utoipa::openapi::schema::Type::Integer))
                                        .format(Some(utoipa::openapi::schema::SchemaFormat::KnownFormat(
                                            utoipa::openapi::schema::KnownFormat::Int32,
                                        ))),
                                ))
                                .build(),
                        ),
                        Some(
                            utoipa::openapi::path::ParameterBuilder::new()
                                .name("filter")
                                .parameter_in(parameter_in_provider().unwrap_or_default())
                                .required(utoipa::openapi::Required::True)
                                .schema(Some(
                                    utoipa::openapi::ObjectBuilder::new()
                                        .schema_type(utoipa::openapi::schema::SchemaType::new(utoipa::openapi::schema::Type::String)),
                                ))
                                .build(),
                        ),
                        Some(
                            utoipa::openapi::path::ParameterBuilder::new()
                                .name("page")
                                .parameter_in(parameter_in_provider().unwrap_or_default())
                                .required(utoipa::openapi::Required::True)
                                .schema(Some(
                                    utoipa::openapi::ObjectBuilder::new()
                                        .schema_type(utoipa::openapi::schema::SchemaType::new(utoipa::openapi::schema::Type::Integer))
                                        .format(Some(utoipa::openapi::schema::SchemaFormat::KnownFormat(
                                            utoipa::openapi::schema::KnownFormat::Int32,
                                        ))),
                                ))
                                .build(),
                        ),
                        Some(
                            utoipa::openapi::path::ParameterBuilder::new()
                                .name("size")
                                .parameter_in(parameter_in_provider().unwrap_or_default())
                                .required(utoipa::openapi::Required::True)
                                .schema(Some(
                                    utoipa::openapi::ObjectBuilder::new()
                                        .schema_type(utoipa::openapi::schema::SchemaType::new(utoipa::openapi::schema::Type::Integer))
                                        .format(Some(utoipa::openapi::schema::SchemaFormat::KnownFormat(
                                            utoipa::openapi::schema::KnownFormat::Int32,
                                        ))),
                                ))
                                .build(),
                        ),
                    ]
                    .into_iter()
                    .filter(Option::is_some)
                    .flatten()
                    .collect()
                }
            }
            impl utoipa::__dev::ComposeSchema for FilterDto {
                fn compose(
                    mut generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
                ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
                    {
                        let mut object = utoipa::openapi::ObjectBuilder::new();
                        object = object
                            .property(
                                "id",
                                utoipa::openapi::ObjectBuilder::new()
                                    .schema_type(utoipa::openapi::schema::SchemaType::new(utoipa::openapi::schema::Type::Integer))
                                    .format(Some(utoipa::openapi::schema::SchemaFormat::KnownFormat(utoipa::openapi::schema::KnownFormat::Int32))),
                            )
                            .required("id");
                        object = object
                            .property(
                                "filter",
                                utoipa::openapi::ObjectBuilder::new()
                                    .schema_type(utoipa::openapi::schema::SchemaType::new(utoipa::openapi::schema::Type::String)),
                            )
                            .required("filter");
                        object = object
                            .property(
                                "page",
                                utoipa::openapi::ObjectBuilder::new()
                                    .schema_type(utoipa::openapi::schema::SchemaType::new(utoipa::openapi::schema::Type::Integer))
                                    .format(Some(utoipa::openapi::schema::SchemaFormat::KnownFormat(utoipa::openapi::schema::KnownFormat::Int32))),
                            )
                            .required("page");
                        object = object
                            .property(
                                "size",
                                utoipa::openapi::ObjectBuilder::new()
                                    .schema_type(utoipa::openapi::schema::SchemaType::new(utoipa::openapi::schema::Type::Integer))
                                    .format(Some(utoipa::openapi::schema::SchemaFormat::KnownFormat(utoipa::openapi::schema::KnownFormat::Int32))),
                            )
                            .required("size");
                        object
                    }
                    .into()
                }
            }
            impl utoipa::ToSchema for FilterDto {
                fn name() -> std::borrow::Cow<'static, str> {
                    std::borrow::Cow::Borrowed("FilterDto")
                }
                fn schemas(schemas: &mut Vec<(String, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>)>) {
                    schemas.extend([]);
                }
            }
            impl nidrs::openapi::ToParamDto for FilterDto {
                fn to_param_dto(dto_type: nidrs::openapi::ParamDtoIn) -> nidrs::openapi::ParamDto {
                    use nidrs::openapi::utoipa;
                    use nidrs::openapi::utoipa::openapi::RefOr;
                    use nidrs::openapi::utoipa::openapi::Schema;
                    use nidrs::openapi::utoipa::IntoParams;
                    use nidrs::openapi::utoipa::ToSchema;
                    let ref_schema: RefOr<Schema> = {
                        let mut generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>> = Vec::new();
                        utoipa::openapi::schema::RefBuilder::new().ref_location_from_schema_name(::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(format_args!("{0}", <Self as utoipa::ToSchema>::name()));
                            res
                        }))
                    }
                    .into();
                    let mut schemas: Vec<(String, RefOr<Schema>)> = <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([(
                            <Self as utoipa::ToSchema>::name().to_string(),
                            {
                                let mut generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>> = Vec::new();
                                <Self as utoipa::PartialSchema>::schema()
                            }
                            .into(),
                        )]),
                    );
                    <Self as utoipa::ToSchema>::schemas(&mut schemas);
                    match dto_type {
                        nidrs::openapi::ParamDtoIn::Param(p) => nidrs::openapi::ParamDto::ParamList(Self::into_params(|| Some(p.clone()))),
                        nidrs::openapi::ParamDtoIn::Body => nidrs::openapi::ParamDto::BodySchema((ref_schema, schemas)),
                        _ => nidrs::openapi::ParamDto::None,
                    }
                }
            }
            pub struct CreateUserResDto {
                pub id: i32,
                pub name: String,
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for CreateUserResDto {
                    fn serialize<__S>(&self, __serializer: __S) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(__serializer, "CreateUserResDto", false as usize + 1 + 1)?;
                        _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "id", &self.id)?;
                        _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "name", &self.name)?;
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
                impl<'de> _serde::Deserialize<'de> for CreateUserResDto {
                    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
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
                            fn expecting(&self, __formatter: &mut _serde::__private::Formatter) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(__formatter, "field identifier")
                            }
                            fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(self, __value: &str) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "id" => _serde::__private::Ok(__Field::__field0),
                                    "name" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"id" => _serde::__private::Ok(__Field::__field0),
                                    b"name" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<CreateUserResDto>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = CreateUserResDto;
                            fn expecting(&self, __formatter: &mut _serde::__private::Formatter) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(__formatter, "struct CreateUserResDto")
                            }
                            #[inline]
                            fn visit_seq<__A>(self, mut __seq: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<i32>(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(_serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct CreateUserResDto with 2 elements",
                                        ));
                                    }
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<String>(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(_serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct CreateUserResDto with 2 elements",
                                        ));
                                    }
                                };
                                _serde::__private::Ok(CreateUserResDto { id: __field0, name: __field1 })
                            }
                            #[inline]
                            fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<i32> = _serde::__private::None;
                                let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("id"));
                                            }
                                            __field0 = _serde::__private::Some(_serde::de::MapAccess::next_value::<i32>(&mut __map)?);
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("name"));
                                            }
                                            __field1 = _serde::__private::Some(_serde::de::MapAccess::next_value::<String>(&mut __map)?);
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => _serde::__private::de::missing_field("id")?,
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => _serde::__private::de::missing_field("name")?,
                                };
                                _serde::__private::Ok(CreateUserResDto { id: __field0, name: __field1 })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["id", "name"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "CreateUserResDto",
                            FIELDS,
                            __Visitor { marker: _serde::__private::PhantomData::<CreateUserResDto>, lifetime: _serde::__private::PhantomData },
                        )
                    }
                }
            };
            #[automatically_derived]
            impl ::core::fmt::Debug for CreateUserResDto {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(f, "CreateUserResDto", "id", &self.id, "name", &&self.name)
                }
            }
            impl nidrs::valid::validator::Validator for CreateUserResDto {
                fn valid(&self) -> nidrs::valid::validator::ValidResult {
                    use nidrs::valid::ruleset;
                    use nidrs::valid::ruleset::*;
                    use nidrs::valid::validator::Rule;
                    return Ok(());
                }
                fn example(&self) -> Vec<serde_json::Value> {
                    ::alloc::vec::Vec::new()
                }
            }
            impl utoipa::IntoParams for CreateUserResDto {
                fn into_params(
                    parameter_in_provider: impl Fn() -> Option<utoipa::openapi::path::ParameterIn>,
                ) -> Vec<utoipa::openapi::path::Parameter> {
                    [
                        Some(
                            utoipa::openapi::path::ParameterBuilder::new()
                                .name("id")
                                .parameter_in(parameter_in_provider().unwrap_or_default())
                                .required(utoipa::openapi::Required::True)
                                .schema(Some(
                                    utoipa::openapi::ObjectBuilder::new()
                                        .schema_type(utoipa::openapi::schema::SchemaType::new(utoipa::openapi::schema::Type::Integer))
                                        .format(Some(utoipa::openapi::schema::SchemaFormat::KnownFormat(
                                            utoipa::openapi::schema::KnownFormat::Int32,
                                        ))),
                                ))
                                .build(),
                        ),
                        Some(
                            utoipa::openapi::path::ParameterBuilder::new()
                                .name("name")
                                .parameter_in(parameter_in_provider().unwrap_or_default())
                                .required(utoipa::openapi::Required::True)
                                .schema(Some(
                                    utoipa::openapi::ObjectBuilder::new()
                                        .schema_type(utoipa::openapi::schema::SchemaType::new(utoipa::openapi::schema::Type::String)),
                                ))
                                .build(),
                        ),
                    ]
                    .into_iter()
                    .filter(Option::is_some)
                    .flatten()
                    .collect()
                }
            }
            impl utoipa::__dev::ComposeSchema for CreateUserResDto {
                fn compose(
                    mut generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
                ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
                    {
                        let mut object = utoipa::openapi::ObjectBuilder::new();
                        object = object
                            .property(
                                "id",
                                utoipa::openapi::ObjectBuilder::new()
                                    .schema_type(utoipa::openapi::schema::SchemaType::new(utoipa::openapi::schema::Type::Integer))
                                    .format(Some(utoipa::openapi::schema::SchemaFormat::KnownFormat(utoipa::openapi::schema::KnownFormat::Int32))),
                            )
                            .required("id");
                        object = object
                            .property(
                                "name",
                                utoipa::openapi::ObjectBuilder::new()
                                    .schema_type(utoipa::openapi::schema::SchemaType::new(utoipa::openapi::schema::Type::String)),
                            )
                            .required("name");
                        object
                    }
                    .into()
                }
            }
            impl utoipa::ToSchema for CreateUserResDto {
                fn name() -> std::borrow::Cow<'static, str> {
                    std::borrow::Cow::Borrowed("CreateUserResDto")
                }
                fn schemas(schemas: &mut Vec<(String, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>)>) {
                    schemas.extend([]);
                }
            }
            impl nidrs::openapi::ToParamDto for CreateUserResDto {
                fn to_param_dto(dto_type: nidrs::openapi::ParamDtoIn) -> nidrs::openapi::ParamDto {
                    use nidrs::openapi::utoipa;
                    use nidrs::openapi::utoipa::openapi::RefOr;
                    use nidrs::openapi::utoipa::openapi::Schema;
                    use nidrs::openapi::utoipa::IntoParams;
                    use nidrs::openapi::utoipa::ToSchema;
                    let ref_schema: RefOr<Schema> = {
                        let mut generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>> = Vec::new();
                        utoipa::openapi::schema::RefBuilder::new().ref_location_from_schema_name(::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(format_args!("{0}", <Self as utoipa::ToSchema>::name()));
                            res
                        }))
                    }
                    .into();
                    let mut schemas: Vec<(String, RefOr<Schema>)> = <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([(
                            <Self as utoipa::ToSchema>::name().to_string(),
                            {
                                let mut generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>> = Vec::new();
                                <Self as utoipa::PartialSchema>::schema()
                            }
                            .into(),
                        )]),
                    );
                    <Self as utoipa::ToSchema>::schemas(&mut schemas);
                    match dto_type {
                        nidrs::openapi::ParamDtoIn::Param(p) => nidrs::openapi::ParamDto::ParamList(Self::into_params(|| Some(p.clone()))),
                        nidrs::openapi::ParamDtoIn::Body => nidrs::openapi::ParamDto::BodySchema((ref_schema, schemas)),
                        _ => nidrs::openapi::ParamDto::None,
                    }
                }
            }
        }
        pub mod service {
            use crate::app::service::AppService;
            use nidrs::macros::injectable;
            use nidrs::Inject;
            use std::sync::{Arc, Mutex};
            pub struct UserService {
                app_service: Inject<AppService>,
                count: Arc<Mutex<i32>>,
            }
            #[automatically_derived]
            impl ::core::default::Default for UserService {
                #[inline]
                fn default() -> UserService {
                    UserService { app_service: ::core::default::Default::default(), count: ::core::default::Default::default() }
                }
            }
            impl nidrs::Service for UserService {
                fn inject(&self, ctx: nidrs::ModuleCtx, module_name: &str) -> nidrs::ModuleCtx {
                    let service = ctx.get_service::<AppService>(&module_name, "AppService");
                    self.app_service.inject(service.clone());
                    ctx
                }
            }
            impl nidrs::ImplMeta for UserService {
                fn __meta(&self) -> nidrs::InnerMeta {
                    let mut meta = nidrs::InnerMeta::new();
                    meta.set("service", "UserService");
                    meta.set_data(nidrs::datasets::ServiceType::from("Service"));
                    meta.set_data(nidrs::datasets::ServiceName::from("UserService"));
                    meta.set("module", "UserModule");
                    meta.set("global", "app");
                    meta
                }
            }
            impl UserService {
                pub fn get_hello_world(&self) -> String {
                    self.app_service.extract().get_hello_world2()
                }
                pub fn get_hello_world2(&self) -> String {
                    let mut count = self.count.lock().unwrap();
                    *count += 1;
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(format_args!("Hello, World! {0}", count));
                        res
                    })
                }
            }
        }
        use crate::app::AppModule;
        use controller::UserController;
        use service::UserService;
        pub struct UserModule;
        #[automatically_derived]
        impl ::core::default::Default for UserModule {
            #[inline]
            fn default() -> UserModule {
                UserModule {}
            }
        }
        impl nidrs::Module for UserModule {
            fn init(self, mut ctx: nidrs::ModuleCtx) -> nidrs::ModuleCtx {
                use nidrs::{Controller, ImplMeta, InterCtx, Interceptor, InterceptorHandler, ModuleCtx, Service, StateCtx};
                if ctx.modules.contains_key("UserModule") {
                    return ctx;
                }
                {
                    ::std::io::_print(format_args!("{0} ", nidrs_extern::colored::Colorize::green("[nidrs]"),));
                };
                {
                    ::std::io::_print(format_args!("Registering module {0}.\n", "UserModule"));
                };
                ctx.modules.insert("UserModule".to_string(), Box::new(self));
                ctx.imports.insert("UserModule".to_string(), Vec::from(["AppModule".to_string()]));
                ctx.append_exports("UserModule", Vec::<&str>::from(["UserService"]), false);
                if ctx.register_controller("UserModule", "UserController", Box::new(std::sync::Arc::new(UserController::default()))) {
                    let t_controller = ctx.get_controller::<UserController>("UserModule", "UserController");
                    ctx = t_controller.__route_get_all(ctx, "UserModule");
                    ctx = t_controller.__route_get_one(ctx, "UserModule");
                    ctx = t_controller.__route_create_user(ctx, "UserModule");
                }
                let svc = std::sync::Arc::new(UserService::default());
                ctx.register_service("UserModule", "UserService", Box::new(svc));
                let mut ctx = AppModule::default().init(ctx);
                let t = ctx.get_service::<UserService>("UserModule", "UserService");
                {
                    ::std::io::_print(format_args!("{0} ", nidrs_extern::colored::Colorize::green("[nidrs]"),));
                };
                {
                    ::std::io::_print(format_args!("Injecting {0}::{1}.\n", "UserModule", "UserService",));
                };
                let ctx = t.inject(ctx, &"UserModule");
                let t = ctx.get_controller::<UserController>("UserModule", "UserController");
                {
                    ::std::io::_print(format_args!("{0} ", nidrs_extern::colored::Colorize::green("[nidrs]"),));
                };
                {
                    ::std::io::_print(format_args!("Injecting {0}::{1}.\n", "UserModule", "UserController",));
                };
                let ctx = t.inject(ctx, &"UserModule");
                ctx
            }
            fn destroy(&self, ctx: &nidrs::ModuleCtx) {
                {
                    ::std::io::_print(format_args!("{0} ", nidrs_extern::colored::Colorize::green("[nidrs]"),));
                };
                {
                    ::std::io::_print(format_args!("Destroying module {0}.\n", "UserModule"));
                };
            }
        }
        impl nidrs::ImplMeta for UserModule {
            fn __meta(&self) -> nidrs::InnerMeta {
                let mut meta = nidrs::InnerMeta::new();
                meta.set("__", true);
                meta.set("service", "UserModule");
                meta.set("module", "UserModule");
                meta.set("global", "app");
                meta
            }
        }
    }
}
mod shared {
    pub mod fn_test {
        use crate::AppResult;
        use nidrs::externs::anyhow;
        use nidrs::externs::axum::http::StatusCode;
        use nidrs::{throw, Exception};
        pub fn fn_test() -> AppResult {
            nidrs::__throw(
                Exception::new(StatusCode::INTERNAL_SERVER_ERROR, anyhow::Error::msg("Error")),
                &::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(format_args!("from {0} line {1}", "examples/hello/src/shared/fn_test.rs", 8usize,));
                    res
                }),
            )?;
            Ok(())
        }
    }
}
use nidrs::externs::axum::{
    error_handling::HandleErrorLayer,
    extract::Request,
    http::StatusCode,
    middleware::{self, Next},
    response::Response,
    BoxError,
};
use nidrs::externs::tower::timeout::TimeoutLayer;
pub use nidrs::AppError;
pub use nidrs::AppResult;
use std::time::Duration;
fn main() {
    let app = nidrs::NidrsFactory::create(app::AppModule);
    let app = app.default_prefix("/api/{version}");
    let app = app.default_version("v1");
    let app = app.default_layer(
        nidrs::externs::tower::ServiceBuilder::new()
            .layer(HandleErrorLayer::new(|error: BoxError| async move {
                if error.is::<nidrs::externs::tower::timeout::error::Elapsed>() {
                    Ok(StatusCode::REQUEST_TIMEOUT)
                } else {
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(format_args!("Unhandled internal error: {0}", error));
                            res
                        }),
                    ))
                }
            }))
            .layer(TimeoutLayer::new(Duration::from_secs(5)))
            .layer(middleware::from_fn(auth)),
    );
    let app = app.listen(3000);
    app.block();
}
pub mod import {
    pub use crate::app::controller::AppController;
    pub use crate::app::interceptor::AppInterceptor;
    pub use crate::app::service::AppService;
    pub use crate::modules::conf::options::ConfOptions;
    pub use crate::modules::conf::service::ConfService;
    pub use crate::modules::user::controller::UserController;
    pub use crate::modules::user::service::UserService;
}
struct CurrentUser {
    pub id: u64,
    pub username: String,
}
#[automatically_derived]
impl ::core::clone::Clone for CurrentUser {
    #[inline]
    fn clone(&self) -> CurrentUser {
        CurrentUser { id: ::core::clone::Clone::clone(&self.id), username: ::core::clone::Clone::clone(&self.username) }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for CurrentUser {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(f, "CurrentUser", "id", &self.id, "username", &&self.username)
    }
}
async fn auth(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    {
        ::std::io::_print(format_args!("auth {0:?}\n", req));
    };
    req.extensions_mut().insert(CurrentUser { id: 1, username: "foo".to_string() });
    Ok(next.run(req).await)
}
extern crate alloc;
