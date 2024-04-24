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
mod app {
    use nidrs_macro::module;
    pub mod controller {
        use std::collections::HashMap;
        use axum::{extract::Query, Json};
        use nidrs::{version, Inject, Meta};
        use nidrs_macro::{controller, get, meta, post, uses};
        use crate::AppResult;
        use super::{dto::Status, service::AppService};
        pub struct AppController {
            inner: std::sync::Arc<AppControllerInner>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for AppController {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "AppController",
                    "inner",
                    &&self.inner,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for AppController {
            #[inline]
            fn default() -> AppController {
                AppController {
                    inner: ::core::default::Default::default(),
                }
            }
        }
        pub struct AppControllerInner {
            app_service: Inject<AppService>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for AppControllerInner {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "AppControllerInner",
                    "app_service",
                    &&self.app_service,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for AppControllerInner {
            #[inline]
            fn default() -> AppControllerInner {
                AppControllerInner {
                    app_service: ::core::default::Default::default(),
                }
            }
        }
        impl Clone for AppController {
            fn clone(&self) -> Self {
                AppController {
                    inner: self.inner.clone(),
                }
            }
        }
        impl std::ops::Deref for AppController {
            type Target = AppControllerInner;
            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }
        impl nidrs::ControllerService for AppController {}
        impl nidrs::Service for AppController {
            fn inject(&self, ctx: nidrs::ModuleCtx) -> nidrs::ModuleCtx {
                let service = ctx
                    .services
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
                let service = service.as_any().downcast_ref::<AppService>().unwrap();
                self.app_service.inject(service.clone());
                ctx
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
        impl nidrs::ImplMeta for AppController {
            fn __meta() -> nidrs::Meta {
                let mut meta = nidrs::Meta::new();
                meta.set("service_name".to_string(), "AppController");
                meta.set("service_type".to_string(), "ControllerService");
                meta
            }
        }
        impl AppController {
            pub async fn get_hello_world(
                &self,
                meta: Meta,
                Query(q): Query<HashMap<String, String>>,
            ) -> AppResult<Status> {
                {
                    ::std::io::_print(format_args!("Query {0:?}\n", q));
                };
                {
                    ::std::io::_print(
                        format_args!("Meta {0:?}\n", meta.get::<&str>("role")),
                    );
                };
                Ok(Status {
                    db: "ok".to_string(),
                    redis: "ok".to_string(),
                })
            }
            pub fn __meta_get_hello_world(&self) -> nidrs::Meta {
                let mut meta = nidrs::Meta::new();
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
            pub fn __meta_get_hello_world2(&self) -> nidrs::Meta {
                let mut meta = nidrs::Meta::new();
                meta
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
            pub fn __meta_post_hello_world(&self) -> nidrs::Meta {
                let mut meta = nidrs::Meta::new();
                meta
            }
        }
    }
    pub mod dto {
        use axum::{
            body::Body, http::{header, StatusCode},
            response::{IntoResponse, Response},
        };
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
        pub enum AppException {
            ServiceException,
        }
    }
    pub mod service {
        use crate::user::service::UserService;
        use nidrs::Inject;
        use nidrs_macro::injectable;
        pub struct AppService {
            inner: std::sync::Arc<AppServiceInner>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for AppService {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "AppService",
                    "inner",
                    &&self.inner,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for AppService {
            #[inline]
            fn default() -> AppService {
                AppService {
                    inner: ::core::default::Default::default(),
                }
            }
        }
        pub struct AppServiceInner {
            user_service: Inject<UserService>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for AppServiceInner {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "AppServiceInner",
                    "user_service",
                    &&self.user_service,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for AppServiceInner {
            #[inline]
            fn default() -> AppServiceInner {
                AppServiceInner {
                    user_service: ::core::default::Default::default(),
                }
            }
        }
        impl Clone for AppService {
            fn clone(&self) -> Self {
                AppService {
                    inner: self.inner.clone(),
                }
            }
        }
        impl std::ops::Deref for AppService {
            type Target = AppServiceInner;
            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }
        impl nidrs::Service for AppService {
            fn inject(&self, ctx: nidrs::ModuleCtx) -> nidrs::ModuleCtx {
                let service = ctx
                    .services
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
                let service = service.as_any().downcast_ref::<UserService>().unwrap();
                self.user_service.inject(service.clone());
                ctx
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
        impl nidrs::ImplMeta for AppService {
            fn __meta() -> nidrs::Meta {
                let mut meta = nidrs::Meta::new();
                meta.set("service_name".to_string(), "AppService");
                meta.set("service_type".to_string(), "Service");
                meta
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
    use crate::user::UserModule;
    use controller::AppController;
    use service::AppService;
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
        fn init(self, mut ctx: nidrs::ModuleCtx) -> nidrs::ModuleCtx {
            use nidrs::{
                Service, ControllerService, InterceptorService, InterCtx, Interceptor,
                ModuleCtx, StateCtx, ImplMeta,
            };
            if ctx.modules.contains_key("AppModule") {
                return ctx;
            }
            ctx.modules.insert("AppModule".to_string(), Box::new(self));
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
            ctx.controllers
                .insert(
                    "AppController".to_string(),
                    Box::new(controller::AppController::default()),
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
                    format_args!("Registering controller {0}.\n", "AppController"),
                );
            };
            let t_controller = ctx.controllers.get("AppController").unwrap();
            let t_controller = t_controller
                .as_any()
                .downcast_ref::<controller::AppController>()
                .unwrap();
            let t_controller = t_controller.clone();
            let mut meta = nidrs::get_meta(&t_controller);
            let t_meta = t_controller.__meta_get_hello_world();
            meta.merge(t_meta);
            let meta = std::sync::Arc::new(meta);
            let version = *meta
                .get::<&str>("version")
                .unwrap_or(&ctx.defaults.default_version);
            let disable_default_prefix = *meta
                .get::<bool>("disable_default_prefix")
                .unwrap_or(&false);
            let path = if disable_default_prefix {
                "/app/hello".to_string()
            } else {
                nidrs::template_format(
                    &{
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "{0}{1}",
                                ctx.defaults.default_prefix,
                                "/app/hello",
                            ),
                        );
                        res
                    },
                    [("version", version)],
                )
            };
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
                        path,
                    ),
                );
            };
            let router = axum::Router::new()
                .route(
                    &path,
                    axum::routing::get(|p1| async move {
                        let mut t_meta = nidrs::Meta::new();
                        t_meta.extend(meta);
                        let p0 = t_meta;
                        t_controller.get_hello_world(p0, p1).await
                    }),
                );
            ctx.routers.push(router);
            let t_controller = ctx.controllers.get("AppController").unwrap();
            let t_controller = t_controller
                .as_any()
                .downcast_ref::<controller::AppController>()
                .unwrap();
            let t_controller = t_controller.clone();
            let mut meta = nidrs::get_meta(&t_controller);
            let t_meta = t_controller.__meta_post_hello_world();
            meta.merge(t_meta);
            let meta = std::sync::Arc::new(meta);
            let version = *meta
                .get::<&str>("version")
                .unwrap_or(&ctx.defaults.default_version);
            let disable_default_prefix = *meta
                .get::<bool>("disable_default_prefix")
                .unwrap_or(&false);
            let path = if disable_default_prefix {
                "/app/hello".to_string()
            } else {
                nidrs::template_format(
                    &{
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "{0}{1}",
                                ctx.defaults.default_prefix,
                                "/app/hello",
                            ),
                        );
                        res
                    },
                    [("version", version)],
                )
            };
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
                        path,
                    ),
                );
            };
            let router = axum::Router::new()
                .route(
                    &path,
                    axum::routing::post(|p0, p1| async move {
                        let mut t_meta = nidrs::Meta::new();
                        t_meta.extend(meta);
                        t_controller.post_hello_world(p0, p1).await
                    }),
                );
            ctx.routers.push(router);
            let t_controller = ctx.controllers.get("AppController").unwrap();
            let t_controller = t_controller
                .as_any()
                .downcast_ref::<controller::AppController>()
                .unwrap();
            let t_controller = t_controller.clone();
            let mut meta = nidrs::get_meta(&t_controller);
            let t_meta = t_controller.__meta_get_hello_world2();
            meta.merge(t_meta);
            let meta = std::sync::Arc::new(meta);
            let version = *meta
                .get::<&str>("version")
                .unwrap_or(&ctx.defaults.default_version);
            let disable_default_prefix = *meta
                .get::<bool>("disable_default_prefix")
                .unwrap_or(&false);
            let path = if disable_default_prefix {
                "/app/hello2".to_string()
            } else {
                nidrs::template_format(
                    &{
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "{0}{1}",
                                ctx.defaults.default_prefix,
                                "/app/hello2",
                            ),
                        );
                        res
                    },
                    [("version", version)],
                )
            };
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
                        path,
                    ),
                );
            };
            let router = axum::Router::new()
                .route(
                    &path,
                    axum::routing::get(|p0| async move {
                        let mut t_meta = nidrs::Meta::new();
                        t_meta.extend(meta);
                        t_controller.get_hello_world2(p0).await
                    }),
                );
            ctx.routers.push(router);
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
                .insert("AppService".to_string(), Box::new(AppService::default()));
            let ctx = UserModule::default().init(ctx);
            let t = ctx.services.get("AppService").unwrap();
            let t = t.as_any().downcast_ref::<AppService>().unwrap();
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
            let ctx = t.inject(ctx);
            let t = ctx.controllers.get("AppController").unwrap();
            let t = t.as_any().downcast_ref::<AppController>().unwrap();
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
            let ctx = t.inject(ctx);
            ctx
        }
        fn destroy(&self, ctx: &nidrs::ModuleCtx) {
            {
                ::std::io::_print(
                    format_args!(
                        "{0} ",
                        nidrs_extern::colored::Colorize::green("[nidrs]"),
                    ),
                );
            };
            {
                ::std::io::_print(format_args!("Destroying module {0}.\n", "AppModule"));
            };
        }
    }
    impl nidrs::ImplMeta for AppModule {
        fn __meta() -> nidrs::Meta {
            let mut meta = nidrs::Meta::new();
            meta.set("module_name".to_string(), "AppModule");
            meta
        }
    }
}
mod conf {
    pub mod options {
        use nidrs_macro::injectable;
        pub struct ConfOptions {
            inner: std::sync::Arc<ConfOptionsInner>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ConfOptions {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "ConfOptions",
                    "inner",
                    &&self.inner,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for ConfOptions {
            #[inline]
            fn default() -> ConfOptions {
                ConfOptions {
                    inner: ::core::default::Default::default(),
                }
            }
        }
        pub struct ConfOptionsInner {
            pub log_level: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ConfOptionsInner {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "ConfOptionsInner",
                    "log_level",
                    &&self.log_level,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for ConfOptionsInner {
            #[inline]
            fn default() -> ConfOptionsInner {
                ConfOptionsInner {
                    log_level: ::core::default::Default::default(),
                }
            }
        }
        impl Clone for ConfOptions {
            fn clone(&self) -> Self {
                ConfOptions {
                    inner: self.inner.clone(),
                }
            }
        }
        impl std::ops::Deref for ConfOptions {
            type Target = ConfOptionsInner;
            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }
        impl nidrs::Service for ConfOptions {
            fn inject(&self, ctx: nidrs::ModuleCtx) -> nidrs::ModuleCtx {
                ctx
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
        impl nidrs::ImplMeta for ConfOptions {
            fn __meta() -> nidrs::Meta {
                let mut meta = nidrs::Meta::new();
                meta.set("service_name".to_string(), "ConfOptions");
                meta.set("service_type".to_string(), "Service");
                meta
            }
        }
    }
    pub mod service {
        use nidrs::{on_module_destroy, Inject};
        use nidrs_macro::{injectable, on_module_init};
        use super::options::ConfOptions;
        pub struct ConfService {
            inner: std::sync::Arc<ConfServiceInner>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ConfService {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "ConfService",
                    "inner",
                    &&self.inner,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for ConfService {
            #[inline]
            fn default() -> ConfService {
                ConfService {
                    inner: ::core::default::Default::default(),
                }
            }
        }
        pub struct ConfServiceInner {
            pub options: Inject<ConfOptions>,
            pub log_level: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ConfServiceInner {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "ConfServiceInner",
                    "options",
                    &self.options,
                    "log_level",
                    &&self.log_level,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for ConfServiceInner {
            #[inline]
            fn default() -> ConfServiceInner {
                ConfServiceInner {
                    options: ::core::default::Default::default(),
                    log_level: ::core::default::Default::default(),
                }
            }
        }
        impl Clone for ConfService {
            fn clone(&self) -> Self {
                ConfService {
                    inner: self.inner.clone(),
                }
            }
        }
        impl std::ops::Deref for ConfService {
            type Target = ConfServiceInner;
            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }
        impl nidrs::Service for ConfService {
            fn inject(&self, ctx: nidrs::ModuleCtx) -> nidrs::ModuleCtx {
                let service = ctx
                    .services
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
                let service = service.as_any().downcast_ref::<ConfOptions>().unwrap();
                self.options.inject(service.clone());
                ctx
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
        impl nidrs::ImplMeta for ConfService {
            fn __meta() -> nidrs::Meta {
                let mut meta = nidrs::Meta::new();
                meta.set("service_name".to_string(), "ConfService");
                meta.set("service_type".to_string(), "Service");
                meta
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
            pub fn on_module_destroy(&self) {
                {
                    ::std::io::_print(format_args!("ConfService destroyed\n"));
                };
            }
        }
    }
    use nidrs::{DynamicModule, Service};
    use nidrs_macro::module;
    pub use options::ConfOptions;
    use service::ConfService;
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
        fn init(self, mut ctx: nidrs::ModuleCtx) -> nidrs::ModuleCtx {
            use nidrs::{
                Service, ControllerService, InterceptorService, InterCtx, Interceptor,
                ModuleCtx, StateCtx, ImplMeta,
            };
            if ctx.modules.contains_key("ConfModule") {
                return ctx;
            }
            ctx.modules.insert("ConfModule".to_string(), Box::new(self));
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
                .insert("ConfService".to_string(), Box::new(ConfService::default()));
            let t = ctx.services.get("ConfService").unwrap();
            let t = t.as_any().downcast_ref::<ConfService>().unwrap();
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
            let ctx = t.inject(ctx);
            let service = ctx.services.get("ConfService").unwrap();
            let service = service.as_any().downcast_ref::<ConfService>().unwrap();
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
                        "Triggering event {0} for {1}.\n",
                        "on_module_init",
                        "ConfService",
                    ),
                );
            };
            service.on_module_init();
            ctx
        }
        fn destroy(&self, ctx: &nidrs::ModuleCtx) {
            let service = ctx.services.get("ConfService").unwrap();
            let service = service.as_any().downcast_ref::<ConfService>().unwrap();
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
                        "Triggering event {0} for {1}.\n",
                        "on_module_destroy",
                        "ConfService",
                    ),
                );
            };
            service.on_module_destroy();
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
                    format_args!("Destroying module {0}.\n", "ConfModule"),
                );
            };
        }
    }
    impl nidrs::ImplMeta for ConfModule {
        fn __meta() -> nidrs::Meta {
            let mut meta = nidrs::Meta::new();
            meta.set("module_name".to_string(), "ConfModule");
            meta
        }
    }
    impl ConfModule {
        pub fn for_root(options: ConfOptions) -> DynamicModule {
            DynamicModule::new().provider(options)
        }
    }
}
mod log {
    use nidrs_macro::module;
    pub mod interceptor {
        use std::fmt::Debug;
        use axum::extract::FromRequest;
        use nidrs::{AnyBody, Inject, InterCtx, Interceptor, IntoAnyBody, StateCtx};
        use nidrs_macro::interceptor;
        use crate::AppResult;
        use super::service::LogService;
        pub struct LogInterceptor {
            inner: std::sync::Arc<LogInterceptorInner>,
        }
        #[automatically_derived]
        impl ::core::default::Default for LogInterceptor {
            #[inline]
            fn default() -> LogInterceptor {
                LogInterceptor {
                    inner: ::core::default::Default::default(),
                }
            }
        }
        pub struct LogInterceptorInner {
            log_service: Inject<LogService>,
        }
        #[automatically_derived]
        impl ::core::default::Default for LogInterceptorInner {
            #[inline]
            fn default() -> LogInterceptorInner {
                LogInterceptorInner {
                    log_service: ::core::default::Default::default(),
                }
            }
        }
        impl Clone for LogInterceptor {
            fn clone(&self) -> Self {
                LogInterceptor {
                    inner: self.inner.clone(),
                }
            }
        }
        impl std::ops::Deref for LogInterceptor {
            type Target = LogInterceptorInner;
            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }
        impl nidrs::InterceptorService for LogInterceptor {}
        impl nidrs::Service for LogInterceptor {
            fn inject(&self, ctx: nidrs::ModuleCtx) -> nidrs::ModuleCtx {
                let service = ctx
                    .services
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
                let service = service.as_any().downcast_ref::<LogService>().unwrap();
                self.log_service.inject(service.clone());
                ctx
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
        impl nidrs::ImplMeta for LogInterceptor {
            fn __meta() -> nidrs::Meta {
                let mut meta = nidrs::Meta::new();
                meta.set("service_name".to_string(), "LogInterceptor");
                meta.set("service_type".to_string(), "InterceptorService");
                meta
            }
        }
        impl<B: FromRequest<StateCtx> + Debug, P: IntoAnyBody> Interceptor<B, P>
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
                    ::std::io::_print(
                        format_args!(
                            "ctx: {0:?}\n",
                            ctx.meta.get::<bool>("disable_default_prefix"),
                        ),
                    );
                };
                self.log_service.log("Before");
                let r: AppResult<AnyBody> = handler(ctx)
                    .await
                    .map(|r| IntoAnyBody::from_serializable(r));
                self.log_service.log("After");
                r
            }
        }
    }
    pub mod service {
        use nidrs_macro::injectable;
        pub struct LogService {
            inner: std::sync::Arc<LogServiceInner>,
        }
        #[automatically_derived]
        impl ::core::default::Default for LogService {
            #[inline]
            fn default() -> LogService {
                LogService {
                    inner: ::core::default::Default::default(),
                }
            }
        }
        pub struct LogServiceInner {}
        #[automatically_derived]
        impl ::core::default::Default for LogServiceInner {
            #[inline]
            fn default() -> LogServiceInner {
                LogServiceInner {}
            }
        }
        impl Clone for LogService {
            fn clone(&self) -> Self {
                LogService {
                    inner: self.inner.clone(),
                }
            }
        }
        impl std::ops::Deref for LogService {
            type Target = LogServiceInner;
            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }
        impl nidrs::Service for LogService {
            fn inject(&self, ctx: nidrs::ModuleCtx) -> nidrs::ModuleCtx {
                ctx
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
        impl nidrs::ImplMeta for LogService {
            fn __meta() -> nidrs::Meta {
                let mut meta = nidrs::Meta::new();
                meta.set("service_name".to_string(), "LogService");
                meta.set("service_type".to_string(), "Service");
                meta
            }
        }
        impl LogService {
            pub fn log(&self, msg: &str) {
                {
                    ::std::io::_print(format_args!("[Log] {0}\n", msg));
                };
            }
        }
    }
    use service::LogService;
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
        fn init(self, mut ctx: nidrs::ModuleCtx) -> nidrs::ModuleCtx {
            use nidrs::{
                Service, ControllerService, InterceptorService, InterCtx, Interceptor,
                ModuleCtx, StateCtx, ImplMeta,
            };
            if ctx.modules.contains_key("LogModule") {
                return ctx;
            }
            ctx.modules.insert("LogModule".to_string(), Box::new(self));
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
                .insert("LogService".to_string(), Box::new(LogService::default()));
            let t = ctx.services.get("LogService").unwrap();
            let t = t.as_any().downcast_ref::<LogService>().unwrap();
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
            let ctx = t.inject(ctx);
            ctx
        }
        fn destroy(&self, ctx: &nidrs::ModuleCtx) {
            {
                ::std::io::_print(
                    format_args!(
                        "{0} ",
                        nidrs_extern::colored::Colorize::green("[nidrs]"),
                    ),
                );
            };
            {
                ::std::io::_print(format_args!("Destroying module {0}.\n", "LogModule"));
            };
        }
    }
    impl nidrs::ImplMeta for LogModule {
        fn __meta() -> nidrs::Meta {
            let mut meta = nidrs::Meta::new();
            meta.set("module_name".to_string(), "LogModule");
            meta
        }
    }
}
mod shared {
    pub mod fn_test {
        use axum::http::StatusCode;
        use nidrs::{throw, Exception};
        use crate::AppResult;
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
mod user {
    use nidrs_macro::module;
    pub mod controller {
        use std::collections::HashMap;
        use axum::extract::Query;
        use nidrs::Inject;
        use nidrs_macro::{controller, get};
        use super::service::UserService;
        pub struct UserController {
            inner: std::sync::Arc<UserControllerInner>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for UserController {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "UserController",
                    "inner",
                    &&self.inner,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for UserController {
            #[inline]
            fn default() -> UserController {
                UserController {
                    inner: ::core::default::Default::default(),
                }
            }
        }
        pub struct UserControllerInner {
            user_service: Inject<UserService>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for UserControllerInner {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "UserControllerInner",
                    "user_service",
                    &&self.user_service,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for UserControllerInner {
            #[inline]
            fn default() -> UserControllerInner {
                UserControllerInner {
                    user_service: ::core::default::Default::default(),
                }
            }
        }
        impl Clone for UserController {
            fn clone(&self) -> Self {
                UserController {
                    inner: self.inner.clone(),
                }
            }
        }
        impl std::ops::Deref for UserController {
            type Target = UserControllerInner;
            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }
        impl nidrs::ControllerService for UserController {}
        impl nidrs::Service for UserController {
            fn inject(&self, ctx: nidrs::ModuleCtx) -> nidrs::ModuleCtx {
                let service = ctx
                    .services
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
                let service = service.as_any().downcast_ref::<UserService>().unwrap();
                self.user_service.inject(service.clone());
                ctx
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
        impl nidrs::ImplMeta for UserController {
            fn __meta() -> nidrs::Meta {
                let mut meta = nidrs::Meta::new();
                meta.set("service_name".to_string(), "UserController");
                meta.set("service_type".to_string(), "ControllerService");
                meta
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
            pub fn __meta_get_hello_world(&self) -> nidrs::Meta {
                let mut meta = nidrs::Meta::new();
                meta
            }
        }
    }
    pub mod service {
        use std::sync::{Arc, Mutex};
        use nidrs::Inject;
        use nidrs_macro::injectable;
        use crate::app::service::AppService;
        pub struct UserService {
            inner: std::sync::Arc<UserServiceInner>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for UserService {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "UserService",
                    "inner",
                    &&self.inner,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for UserService {
            #[inline]
            fn default() -> UserService {
                UserService {
                    inner: ::core::default::Default::default(),
                }
            }
        }
        pub struct UserServiceInner {
            app_service: Inject<AppService>,
            count: Arc<Mutex<i32>>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for UserServiceInner {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "UserServiceInner",
                    "app_service",
                    &self.app_service,
                    "count",
                    &&self.count,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for UserServiceInner {
            #[inline]
            fn default() -> UserServiceInner {
                UserServiceInner {
                    app_service: ::core::default::Default::default(),
                    count: ::core::default::Default::default(),
                }
            }
        }
        impl Clone for UserService {
            fn clone(&self) -> Self {
                UserService {
                    inner: self.inner.clone(),
                }
            }
        }
        impl std::ops::Deref for UserService {
            type Target = UserServiceInner;
            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }
        impl nidrs::Service for UserService {
            fn inject(&self, ctx: nidrs::ModuleCtx) -> nidrs::ModuleCtx {
                let service = ctx
                    .services
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
                let service = service.as_any().downcast_ref::<AppService>().unwrap();
                self.app_service.inject(service.clone());
                ctx
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
        impl nidrs::ImplMeta for UserService {
            fn __meta() -> nidrs::Meta {
                let mut meta = nidrs::Meta::new();
                meta.set("service_name".to_string(), "UserService");
                meta.set("service_type".to_string(), "Service");
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
                {
                    let res = ::alloc::fmt::format(
                        format_args!("Hello, World! {0}", count),
                    );
                    res
                }
            }
        }
    }
    use crate::app::AppModule;
    use controller::UserController;
    use service::UserService;
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
        fn init(self, mut ctx: nidrs::ModuleCtx) -> nidrs::ModuleCtx {
            use nidrs::{
                Service, ControllerService, InterceptorService, InterCtx, Interceptor,
                ModuleCtx, StateCtx, ImplMeta,
            };
            if ctx.modules.contains_key("UserModule") {
                return ctx;
            }
            ctx.modules.insert("UserModule".to_string(), Box::new(self));
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
            ctx.controllers
                .insert(
                    "UserController".to_string(),
                    Box::new(controller::UserController::default()),
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
                    format_args!("Registering controller {0}.\n", "UserController"),
                );
            };
            let t_controller = ctx.controllers.get("UserController").unwrap();
            let t_controller = t_controller
                .as_any()
                .downcast_ref::<controller::UserController>()
                .unwrap();
            let t_controller = t_controller.clone();
            let mut meta = nidrs::get_meta(&t_controller);
            let t_meta = t_controller.__meta_get_hello_world();
            meta.merge(t_meta);
            let meta = std::sync::Arc::new(meta);
            let version = *meta
                .get::<&str>("version")
                .unwrap_or(&ctx.defaults.default_version);
            let disable_default_prefix = *meta
                .get::<bool>("disable_default_prefix")
                .unwrap_or(&false);
            let path = if disable_default_prefix {
                "/user/hello".to_string()
            } else {
                nidrs::template_format(
                    &{
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "{0}{1}",
                                ctx.defaults.default_prefix,
                                "/user/hello",
                            ),
                        );
                        res
                    },
                    [("version", version)],
                )
            };
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
                        path,
                    ),
                );
            };
            let router = axum::Router::new()
                .route(
                    &path,
                    axum::routing::get(|p0| async move {
                        let mut t_meta = nidrs::Meta::new();
                        t_meta.extend(meta);
                        t_controller.get_hello_world(p0).await
                    }),
                );
            ctx.routers.push(router);
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
                .insert("UserService".to_string(), Box::new(UserService::default()));
            let ctx = AppModule::default().init(ctx);
            let t = ctx.services.get("UserService").unwrap();
            let t = t.as_any().downcast_ref::<UserService>().unwrap();
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
            let ctx = t.inject(ctx);
            let t = ctx.controllers.get("UserController").unwrap();
            let t = t.as_any().downcast_ref::<UserController>().unwrap();
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
                ::std::io::_print(format_args!("Injecting {0}.\n", "UserController"));
            };
            let ctx = t.inject(ctx);
            ctx
        }
        fn destroy(&self, ctx: &nidrs::ModuleCtx) {
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
                    format_args!("Destroying module {0}.\n", "UserModule"),
                );
            };
        }
    }
    impl nidrs::ImplMeta for UserModule {
        fn __meta() -> nidrs::Meta {
            let mut meta = nidrs::Meta::new();
            meta.set("module_name".to_string(), "UserModule");
            meta
        }
    }
}
pub use nidrs::AppError;
pub use nidrs::AppResult;
fn main() {
    let app = nidrs::NidrsFactory::create(app::AppModule);
    app.listen(3000);
}
extern crate alloc;
