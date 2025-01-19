#![feature(print_internals)]
#![feature(panic_internals)]
#![feature(alloc)]
#![feature(fmt_helpers_for_derive)]
#![allow(warnings, unused)]
#![feature(hint_must_use)]
#![feature(liballoc_internals)]
// >>Push: Global("app") -- [None]
//  CMETA: []
// >>Push: Module("DieselModule") -- [None]
// >>Push: Service("SqlitePoolManager") -- [Some(String("DieselModule"))]
//  CMETA: ["ServiceType"]
//  CMETA: ["ServiceName"]
// service_derive "SqlitePoolManager"
// << Pop: Some(Service("SqlitePoolManager")) ["ServiceType", "ServiceName", "service", "module", "global"]

// >>Push: Service("MysqlPoolManager") -- [Some(String("DieselModule"))]
//  CMETA: ["ServiceType"]
//  CMETA: ["ServiceName"]
// service_derive "MysqlPoolManager"
// << Pop: Some(Service("MysqlPoolManager")) ["service", "ServiceType", "ServiceName", "module", "global"]

// >>Push: Service("DieselService") -- [Some(String("DieselModule"))]
//  CMETA: ["ServiceType"]
//  CMETA: ["ServiceName"]
// service_derive "DieselService"
// << Pop: Some(Service("DieselService")) ["ServiceName", "service", "ServiceType", "module", "global"]

// >>Push: Service("DieselModule") -- [Some(String("DieselModule"))]
//  CMETA: ["Global"]
//  CMETA: ["__"]
// module "DieselModule"
// >>Push: Global("app") -- [None]
//  CMETA: []
// >>Push: Module("AppModule") -- [None]
// >>Push: Service("AppController") -- [Some(String("AppModule"))]
//  CMETA: ["ServiceType"]
//  CMETA: ["ServiceName"]
//  CMETA: ["ControllerPath"]
// service_derive "AppController"
// >>Push: Handler("get_hello_world") -- [Some(String("AppModule"))]
//  CMETA: ["RouterName"]
//  CMETA: ["RouterMethod"]
//  CMETA: ["RouterPath"]
// route_derive "get_hello_world"
// << Pop: Some(Handler("get_hello_world")) ["handler", "RouterName", "RouterMethod", "RouterPath", "ServiceName", "ServiceType", "service", "ControllerPath", "module", "global"]

// << Pop: Some(Service("AppController")) ["ServiceName", "ServiceType", "service", "ControllerPath", "module", "global"]

// >>Push: Service("AppService") -- [Some(String("AppModule"))]
//  CMETA: ["ServiceType"]
//  CMETA: ["ServiceName"]
// service_derive "AppService"
// << Pop: Some(Service("AppService")) ["ServiceName", "service", "ServiceType", "module", "global"]

// >>Push: Service("AppModule") -- [Some(String("AppModule"))]
//  CMETA: ["__"]
// module "AppModule"
// << Pop: Some(Service("AppModule")) ["service", "__", "module", "global"]

// >>Push: Service("UserEntity") -- [Some(String("AppModule"))]
//  CMETA: ["ServiceType"]
//  CMETA: ["ServiceName"]
// service_derive "UserEntity"
// << Pop: Some(Service("UserEntity")) ["ServiceType", "service", "ServiceName", "module", "global"]

// << Pop: Some(Module("AppModule")) ["module", "global"]

// >>Push: Module("UserModule") -- [None]
// >>Push: Service("UserController") -- [Some(String("UserModule"))]
//  CMETA: ["ServiceType"]
//  CMETA: ["ServiceName"]
//  CMETA: ["ControllerPath"]
// service_derive "UserController"
// >>Push: Handler("get_user_all") -- [Some(String("UserModule"))]
//  CMETA: ["RouterName"]
//  CMETA: ["RouterMethod"]
//  CMETA: ["RouterPath"]
// route_derive "get_user_all"
// << Pop: Some(Handler("get_user_all")) ["handler", "RouterName", "RouterPath", "RouterMethod", "ServiceType", "service", "ControllerPath", "ServiceName", "module", "global"]

// >>Push: Handler("get_user_by_id") -- [Some(String("UserModule"))]
//  CMETA: ["RouterName"]
//  CMETA: ["RouterMethod"]
//  CMETA: ["RouterPath"]
// route_derive "get_user_by_id"
// << Pop: Some(Handler("get_user_by_id")) ["RouterPath", "RouterName", "RouterMethod", "handler", "ServiceType", "service", "ControllerPath", "ServiceName", "module", "global"]

// >>Push: Handler("create_user") -- [Some(String("UserModule"))]
//  CMETA: ["RouterOut", "RouterIn"]
//  CMETA: ["RouterName"]
//  CMETA: ["RouterMethod"]
//  CMETA: ["RouterPath"]
// route_derive "create_user"
// << Pop: Some(Handler("create_user")) ["RouterPath", "handler", "RouterOut", "RouterIn", "RouterName", "RouterMethod", "ServiceType", "service", "ControllerPath", "ServiceName", "module", "global"]

// << Pop: Some(Service("UserController")) ["ServiceType", "service", "ControllerPath", "ServiceName", "module", "global"]

// >>Push: Service("UserService") -- [Some(String("UserModule"))]
//  CMETA: ["ServiceType"]
//  CMETA: ["ServiceName"]
// service_derive "UserService"
// << Pop: Some(Service("UserService")) ["ServiceName", "service", "ServiceType", "module", "global"]

// >>Push: Service("UserModule") -- [Some(String("UserModule"))]
//  CMETA: ["__"]
// module "UserModule"
// << Pop: Some(Service("UserModule")) ["service", "__", "module", "global"]
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod app {
    use nidrs_macro::module;
    pub mod controller {
        use super::service::AppService;
        use crate::AppResult;
        use axum::extract::Query;
        use nidrs::externs::axum;
        use nidrs::{Inject, Meta};
        use nidrs_macro::{controller, get};
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
                meta.set_data(nidrs::datasets::ServiceName::from("AppController"));
                meta.set_data(nidrs::datasets::ServiceType::from("Controller"));
                meta.set("service", "AppController");
                meta.set_data(nidrs::datasets::ControllerPath::from("/app"));
                meta.set("module", "AppModule");
                meta.set("global", "app");
                meta
            }
        }
        impl AppController {
            pub async fn get_hello_world(&self, meta: Meta, Query(q): Query<HashMap<String, String>>) -> AppResult<String> {
                {
                    ::std::io::_print(format_args!("Query {0:?}\n", q));
                };
                {
                    ::std::io::_print(format_args!("Meta {0:?}\n", meta.get::<&str>("role")));
                };
                Ok(self.app_service.get_hello_world2())
            }
            pub fn __meta_get_hello_world(&self) -> nidrs::InnerMeta {
                let mut meta = nidrs::InnerMeta::new();
                meta.set("handler", "get_hello_world");
                meta.set_data(nidrs::datasets::RouterName::from("get_hello_world"));
                meta.set_data(nidrs::datasets::RouterMethod::from("get"));
                meta.set_data(nidrs::datasets::RouterPath::from("/hello"));
                meta.set_data(nidrs::datasets::ServiceName::from("AppController"));
                meta.set_data(nidrs::datasets::ServiceType::from("Controller"));
                meta.set("service", "AppController");
                meta.set_data(nidrs::datasets::ControllerPath::from("/app"));
                meta.set("module", "AppModule");
                meta.set("global", "app");
                meta
            }
            pub fn __route_get_hello_world(&self, mut ctx: nidrs::ModuleCtx, module: &str) -> nidrs::ModuleCtx {
                use axum::response::IntoResponse;
                use nidrs::externs::axum;
                use nidrs::externs::axum::{extract::Query, Json};
                use nidrs::externs::meta::{InnerMeta, Meta};
                use nidrs::externs::serde_json::Value;
                use nidrs::Interceptor;
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
                    .route_layer(nidrs::externs::axum::Extension(meta.clone()));
                ctx.routers.push(nidrs::MetaRouter::new(router, meta));
                ctx
            }
        }
    }
    pub mod dto {
        use nidrs::externs::axum::{
            body::Body,
            http::{header, StatusCode},
            response::{IntoResponse, Response},
        };
        use nidrs::externs::serde;
        use nidrs::externs::serde_json;
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
                fn serialize<__S>(&self, __serializer: __S) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(__serializer, "Status", false as usize + 1 + 1)?;
                    _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "db", &self.db)?;
                    _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "redis", &self.redis)?;
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
                                "db" => _serde::__private::Ok(__Field::__field0),
                                "redis" => _serde::__private::Ok(__Field::__field1),
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
                                    return _serde::__private::Err(_serde::de::Error::invalid_length(0usize, &"struct Status with 2 elements"));
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<String>(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(_serde::de::Error::invalid_length(1usize, &"struct Status with 2 elements"));
                                }
                            };
                            _serde::__private::Ok(Status { db: __field0, redis: __field1 })
                        }
                        #[inline]
                        fn visit_map<__A>(self, mut __map: __A) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
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
                            _serde::__private::Ok(Status { db: __field0, redis: __field1 })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["db", "redis"];
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
                ::core::fmt::Formatter::debug_struct_field2_finish(f, "Status", "db", &self.db, "redis", &&self.redis)
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
    }
    pub mod exception {
        pub enum AppException {
            ServiceException(String),
        }
    }
    pub mod service {
        use crate::modules::user::service::UserService;
        use nidrs::Inject;
        use nidrs_macro::injectable;
        pub struct AppService {
            user_service: Inject<UserService>,
        }
        #[automatically_derived]
        impl ::core::default::Default for AppService {
            #[inline]
            fn default() -> AppService {
                AppService { user_service: ::core::default::Default::default() }
            }
        }
        impl nidrs::Service for AppService {
            fn inject(&self, ctx: nidrs::ModuleCtx, module_name: &str) -> nidrs::ModuleCtx {
                let service = ctx.get_service::<UserService>(&module_name, "UserService");
                self.user_service.inject(service.clone());
                ctx
            }
        }
        impl nidrs::ImplMeta for AppService {
            fn __meta(&self) -> nidrs::InnerMeta {
                let mut meta = nidrs::InnerMeta::new();
                meta.set_data(nidrs::datasets::ServiceName::from("AppService"));
                meta.set("service", "AppService");
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
    use crate::modules::user::UserModule;
    use controller::AppController;
    use nidrs_diesel::mysql::MysqlPoolManager;
    use nidrs_diesel::sqlite::SqlitePoolManager;
    use nidrs_diesel::DieselModule as DieselModule1;
    use nidrs_diesel::DieselModule as DieselModule2;
    use nidrs_diesel::DieselOptions;
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
            ctx.imports
                .insert("AppModule".to_string(), Vec::from(["DieselModule1".to_string(), "DieselModule2".to_string(), "UserModule".to_string()]));
            ctx.append_exports("AppModule", Vec::<&str>::from(["AppService"]), false);
            if ctx.register_controller("AppModule", "AppController", Box::new(std::sync::Arc::new(AppController::default()))) {
                let t_controller = ctx.get_controller::<AppController>("AppModule", "AppController");
                ctx = t_controller.__route_get_hello_world(ctx, "AppModule");
            }
            let svc = std::sync::Arc::new(AppService::default());
            ctx.register_service("AppModule", "AppService", Box::new(svc));
            let mut dyn_module =
                DieselModule1::for_root(DieselOptions::new(MysqlPoolManager::new("mysql://root:12345678@127.0.0.1/hello-diesel")).with_name("Mysql"));
            let mut dyn_module_wrap = dyn_module.module.take().unwrap();
            let mut dyn_module_services = dyn_module.services;
            dyn_module_services.drain().for_each(|(k, v)| {
                ctx.register_service("DieselModule1", &k, v);
            });
            let mut dyn_module_exports = dyn_module.exports;
            ctx.append_exports(
                "DieselModule1",
                dyn_module_exports,
                nidrs::get_meta_by_type::<DieselModule1>().get_data::<nidrs::datasets::Global>().unwrap_or(&nidrs::datasets::Global(false)).value(),
            );
            let mut ctx = dyn_module_wrap.init(ctx);
            let mut dyn_module = DieselModule2::for_root(DieselOptions::new(SqlitePoolManager::new("file:db.sqlite3")).with_name("Sqlite"));
            let mut dyn_module_wrap = dyn_module.module.take().unwrap();
            let mut dyn_module_services = dyn_module.services;
            dyn_module_services.drain().for_each(|(k, v)| {
                ctx.register_service("DieselModule2", &k, v);
            });
            let mut dyn_module_exports = dyn_module.exports;
            ctx.append_exports(
                "DieselModule2",
                dyn_module_exports,
                nidrs::get_meta_by_type::<DieselModule2>().get_data::<nidrs::datasets::Global>().unwrap_or(&nidrs::datasets::Global(false)).value(),
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
mod models {
    pub mod entities {
        pub mod user {
            use crate::models::schema::users;
            use chrono::NaiveDateTime;
            use diesel::prelude::*;
            use diesel_async::RunQueryDsl;
            use nidrs::externs::serde;
            use nidrs::{injectable, AppResult, Inject};
            use nidrs_diesel::mysql::MysqlPoolManager as Mysql;
            use nidrs_diesel::sqlite::SqlitePoolManager as Sqlite;
            use nidrs_diesel::AsyncQuery;
            use serde::Serialize;
            #[diesel(table_name = users)]
            pub struct User {
                pub id: u32,
                pub name: String,
                pub updated_at: NaiveDateTime,
                pub created_at: NaiveDateTime,
            }
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                use diesel::expression::Selectable;
                impl<__DB: diesel::backend::Backend> Selectable<__DB> for User {
                    type SelectExpression = (users::id, users::name, users::updated_at, users::created_at);
                    fn construct_selection() -> Self::SelectExpression {
                        (users::id, users::name, users::updated_at, users::created_at)
                    }
                }
            };
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                use diesel::deserialize::{self, FromStaticSqlRow, Queryable};
                use diesel::row::{Field as _, Row as _};
                use std::convert::TryInto;
                impl<__DB: diesel::backend::Backend, __ST0, __ST1, __ST2, __ST3> Queryable<(__ST0, __ST1, __ST2, __ST3), __DB> for User
                where
                    (u32, String, NaiveDateTime, NaiveDateTime): FromStaticSqlRow<(__ST0, __ST1, __ST2, __ST3), __DB>,
                {
                    type Row = (u32, String, NaiveDateTime, NaiveDateTime);
                    fn build(row: Self::Row) -> deserialize::Result<Self> {
                        Ok(Self { id: row.0.try_into()?, name: row.1.try_into()?, updated_at: row.2.try_into()?, created_at: row.3.try_into()? })
                    }
                }
            };
            #[automatically_derived]
            impl ::core::fmt::Debug for User {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field4_finish(
                        f,
                        "User",
                        "id",
                        &self.id,
                        "name",
                        &self.name,
                        "updated_at",
                        &self.updated_at,
                        "created_at",
                        &&self.created_at,
                    )
                }
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for User {
                    fn serialize<__S>(&self, __serializer: __S) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(__serializer, "User", false as usize + 1 + 1 + 1 + 1)?;
                        _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "id", &self.id)?;
                        _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "name", &self.name)?;
                        _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "updated_at", &self.updated_at)?;
                        _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "created_at", &self.created_at)?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            #[diesel(table_name = users)]
            pub struct NewUser {
                pub name: String,
            }
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                use diesel::insertable::Insertable;
                use diesel::internal::derives::insertable::UndecoratedInsertRecord;
                use diesel::prelude::*;
                #[allow(unused_qualifications)]
                impl Insertable<users::table> for NewUser {
                    type Values = <(std::option::Option<diesel::dsl::Eq<users::name, String>>,) as Insertable<users::table>>::Values;
                    fn values(self) -> <(std::option::Option<diesel::dsl::Eq<users::name, String>>,) as Insertable<users::table>>::Values {
                        (std::option::Option::Some(users::name.eq(self.name)),).values()
                    }
                }
                #[allow(unused_qualifications)]
                impl<'insert> Insertable<users::table> for &'insert NewUser {
                    type Values = <(std::option::Option<diesel::dsl::Eq<users::name, &'insert String>>,) as Insertable<users::table>>::Values;
                    fn values(self) -> <(std::option::Option<diesel::dsl::Eq<users::name, &'insert String>>,) as Insertable<users::table>>::Values {
                        (std::option::Option::Some(users::name.eq(&self.name)),).values()
                    }
                }
                impl UndecoratedInsertRecord<users::table> for NewUser {}
            };
            pub struct UserEntity {
                mysql_pool: Inject<Mysql>,
                sqlite_pool: Inject<Sqlite>,
            }
            #[automatically_derived]
            impl ::core::default::Default for UserEntity {
                #[inline]
                fn default() -> UserEntity {
                    UserEntity { mysql_pool: ::core::default::Default::default(), sqlite_pool: ::core::default::Default::default() }
                }
            }
            impl nidrs::Service for UserEntity {
                fn inject(&self, ctx: nidrs::ModuleCtx, module_name: &str) -> nidrs::ModuleCtx {
                    let service = ctx.get_service::<Mysql>(&module_name, "Mysql");
                    self.mysql_pool.inject(service.clone());
                    let service = ctx.get_service::<Sqlite>(&module_name, "Sqlite");
                    self.sqlite_pool.inject(service.clone());
                    ctx
                }
            }
            impl nidrs::ImplMeta for UserEntity {
                fn __meta(&self) -> nidrs::InnerMeta {
                    let mut meta = nidrs::InnerMeta::new();
                    meta.set_data(nidrs::datasets::ServiceType::from("Service"));
                    meta.set("service", "UserEntity");
                    meta.set_data(nidrs::datasets::ServiceName::from("UserEntity"));
                    meta.set("module", "AppModule");
                    meta.set("global", "app");
                    meta
                }
            }
            impl UserEntity {
                pub async fn all(&self) -> AppResult<Vec<User>> {
                    let mut conn = self.mysql_pool.get().await?;
                    let result = users::table.load::<User>(&mut conn).await.unwrap();
                    Ok(result)
                }
                pub async fn create(&self, name: String) -> AppResult<usize> {
                    self.create_sqlite(name.clone()).await?;
                    let mut conn = self.mysql_pool.get().await?;
                    let new_user = NewUser { name };
                    let result = diesel::insert_into(users::table).values(&new_user).execute(&mut conn).await.unwrap();
                    Ok(result)
                }
                pub async fn create_sqlite(&self, name: String) -> AppResult<usize> {
                    let mut conn = self.sqlite_pool.get().await?;
                    let new_user = NewUser { name };
                    let result = diesel::insert_into(users::table).values(&new_user).execute(&mut conn).await.unwrap();
                    Ok(result)
                }
                pub async fn update(&self, id: u32, name: String) -> AppResult<usize> {
                    let mut conn = self.mysql_pool.get().await?;
                    let result = diesel::update(users::table.find(id)).set(users::name.eq(name)).execute(&mut conn).await.unwrap();
                    Ok(result)
                }
                pub async fn find_by_id(&self, id: u32) -> AppResult<User> {
                    let mut conn = self.mysql_pool.get().await?;
                    let result = users::table.find(id).first::<User>(&mut conn).await.unwrap();
                    Ok(result)
                }
                pub async fn remove_by_id(&self, id: u32) -> AppResult<usize> {
                    let mut conn = self.mysql_pool.get().await?;
                    let result = diesel::delete(users::table.find(id)).execute(&mut conn).await.unwrap();
                    Ok(result)
                }
            }
        }
    }
    pub mod schema {
        #[allow(unused_imports, dead_code, unreachable_pub, unused_qualifications)]
        pub mod users {
            pub use self::columns::*;
            use ::diesel;
            use diesel::sql_types::*;
            /// Re-exports all of the columns of this table, as well as the
            /// table struct renamed to the module name. This is meant to be
            /// glob imported for functions which only deal with one table.
            pub mod dsl {
                pub use super::columns::created_at;
                pub use super::columns::id;
                pub use super::columns::name;
                pub use super::columns::updated_at;
                pub use super::table as users;
            }
            #[allow(non_upper_case_globals, dead_code)]
            /// A tuple of all of the columns on this table
            pub const all_columns: (id, name, created_at, updated_at) = (id, name, created_at, updated_at);
            #[allow(non_camel_case_types)]
            /// The actual table struct
            ///
            /// This is the type which provides the base methods of the query
            /// builder, such as `.select` and `.filter`.
            pub struct table;
            #[automatically_derived]
            #[allow(non_camel_case_types)]
            impl ::core::fmt::Debug for table {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(f, "table")
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types)]
            impl ::core::clone::Clone for table {
                #[inline]
                fn clone(&self) -> table {
                    *self
                }
            }
            #[automatically_derived]
            #[allow(non_camel_case_types)]
            impl ::core::marker::Copy for table {}
            #[allow(unused_imports)]
            const _: () = {
                use diesel;
                use diesel::query_builder::QueryId;
                #[allow(non_camel_case_types)]
                impl QueryId for table {
                    type QueryId = table;
                    const HAS_STATIC_QUERY_ID: bool = true;
                }
            };
            #[automatically_derived]
            #[allow(non_camel_case_types)]
            impl ::core::default::Default for table {
                #[inline]
                fn default() -> table {
                    table {}
                }
            }
            impl table {
                #[allow(dead_code)]
                /// Represents `table_name.*`, which is sometimes necessary
                /// for efficient count queries. It cannot be used in place of
                /// `all_columns`
                pub fn star(&self) -> star {
                    star
                }
            }
            /// The SQL type of all of the columns on this table
            pub type SqlType = (Unsigned<Integer>, Varchar, Timestamp, Timestamp);
            /// Helper type for representing a boxed query from this table
            pub type BoxedQuery<'a, DB, ST = SqlType> =
                diesel::internal::table_macro::BoxedSelectStatement<'a, ST, diesel::internal::table_macro::FromClause<table>, DB>;
            impl diesel::QuerySource for table {
                type FromClause = diesel::internal::table_macro::StaticQueryFragmentInstance<table>;
                type DefaultSelection = <Self as diesel::Table>::AllColumns;
                fn from_clause(&self) -> Self::FromClause {
                    diesel::internal::table_macro::StaticQueryFragmentInstance::new()
                }
                fn default_selection(&self) -> Self::DefaultSelection {
                    use diesel::Table;
                    Self::all_columns()
                }
            }
            impl<DB> diesel::query_builder::QueryFragment<DB> for table
            where
                DB: diesel::backend::Backend,
                <table as diesel::internal::table_macro::StaticQueryFragment>::Component: diesel::query_builder::QueryFragment<DB>,
            {
                fn walk_ast<'b>(&'b self, __diesel_internal_pass: diesel::query_builder::AstPass<'_, 'b, DB>) -> diesel::result::QueryResult<()> {
                    <table as diesel::internal::table_macro::StaticQueryFragment>::STATIC_COMPONENT.walk_ast(__diesel_internal_pass)
                }
            }
            impl diesel::internal::table_macro::StaticQueryFragment for table {
                type Component = diesel::internal::table_macro::Identifier<'static>;
                const STATIC_COMPONENT: &'static Self::Component = &diesel::internal::table_macro::Identifier("users");
            }
            impl diesel::query_builder::AsQuery for table {
                type SqlType = SqlType;
                type Query = diesel::internal::table_macro::SelectStatement<diesel::internal::table_macro::FromClause<Self>>;
                fn as_query(self) -> Self::Query {
                    diesel::internal::table_macro::SelectStatement::simple(self)
                }
            }
            impl diesel::Table for table {
                type PrimaryKey = id;
                type AllColumns = (id, name, created_at, updated_at);
                fn primary_key(&self) -> Self::PrimaryKey {
                    id
                }
                fn all_columns() -> Self::AllColumns {
                    (id, name, created_at, updated_at)
                }
            }
            impl diesel::associations::HasTable for table {
                type Table = Self;
                fn table() -> Self::Table {
                    table
                }
            }
            impl diesel::query_builder::IntoUpdateTarget for table {
                type WhereClause = <<Self as diesel::query_builder::AsQuery>::Query as diesel::query_builder::IntoUpdateTarget>::WhereClause;
                fn into_update_target(self) -> diesel::query_builder::UpdateTarget<Self::Table, Self::WhereClause> {
                    use diesel::query_builder::AsQuery;
                    let q: diesel::internal::table_macro::SelectStatement<diesel::internal::table_macro::FromClause<table>> = self.as_query();
                    q.into_update_target()
                }
            }
            impl diesel::query_source::AppearsInFromClause<table> for table {
                type Count = diesel::query_source::Once;
            }
            impl<S> diesel::internal::table_macro::AliasAppearsInFromClause<S, table> for table
            where
                S: diesel::query_source::AliasSource<Target = table>,
            {
                type Count = diesel::query_source::Never;
            }
            impl<S1, S2> diesel::internal::table_macro::AliasAliasAppearsInFromClause<table, S2, S1> for table
            where
                S1: diesel::query_source::AliasSource<Target = table>,
                S2: diesel::query_source::AliasSource<Target = table>,
                S1: diesel::internal::table_macro::AliasAliasAppearsInFromClauseSameTable<S2, table>,
            {
                type Count = <S1 as diesel::internal::table_macro::AliasAliasAppearsInFromClauseSameTable<S2, table>>::Count;
            }
            impl<S> diesel::query_source::AppearsInFromClause<diesel::query_source::Alias<S>> for table
            where
                S: diesel::query_source::AliasSource,
            {
                type Count = diesel::query_source::Never;
            }
            impl<S, C> diesel::internal::table_macro::FieldAliasMapperAssociatedTypesDisjointnessTrick<table, S, C> for table
            where
                S: diesel::query_source::AliasSource<Target = table> + ::std::clone::Clone,
                C: diesel::query_source::Column<Table = table>,
            {
                type Out = diesel::query_source::AliasedField<S, C>;
                fn map(__diesel_internal_column: C, __diesel_internal_alias: &diesel::query_source::Alias<S>) -> Self::Out {
                    __diesel_internal_alias.field(__diesel_internal_column)
                }
            }
            impl diesel::query_source::AppearsInFromClause<table> for diesel::internal::table_macro::NoFromClause {
                type Count = diesel::query_source::Never;
            }
            impl<Left, Right, Kind> diesel::JoinTo<diesel::internal::table_macro::Join<Left, Right, Kind>> for table
            where
                diesel::internal::table_macro::Join<Left, Right, Kind>: diesel::JoinTo<table>,
                Left: diesel::query_source::QuerySource,
                Right: diesel::query_source::QuerySource,
            {
                type FromClause = diesel::internal::table_macro::Join<Left, Right, Kind>;
                type OnClause = <diesel::internal::table_macro::Join<Left, Right, Kind> as diesel::JoinTo<table>>::OnClause;
                fn join_target(__diesel_internal_rhs: diesel::internal::table_macro::Join<Left, Right, Kind>) -> (Self::FromClause, Self::OnClause) {
                    let (_, __diesel_internal_on_clause) = diesel::internal::table_macro::Join::join_target(table);
                    (__diesel_internal_rhs, __diesel_internal_on_clause)
                }
            }
            impl<Join, On> diesel::JoinTo<diesel::internal::table_macro::JoinOn<Join, On>> for table
            where
                diesel::internal::table_macro::JoinOn<Join, On>: diesel::JoinTo<table>,
            {
                type FromClause = diesel::internal::table_macro::JoinOn<Join, On>;
                type OnClause = <diesel::internal::table_macro::JoinOn<Join, On> as diesel::JoinTo<table>>::OnClause;
                fn join_target(__diesel_internal_rhs: diesel::internal::table_macro::JoinOn<Join, On>) -> (Self::FromClause, Self::OnClause) {
                    let (_, __diesel_internal_on_clause) = diesel::internal::table_macro::JoinOn::join_target(table);
                    (__diesel_internal_rhs, __diesel_internal_on_clause)
                }
            }
            impl<F, S, D, W, O, L, Of, G>
                diesel::JoinTo<diesel::internal::table_macro::SelectStatement<diesel::internal::table_macro::FromClause<F>, S, D, W, O, L, Of, G>>
                for table
            where
                diesel::internal::table_macro::SelectStatement<diesel::internal::table_macro::FromClause<F>, S, D, W, O, L, Of, G>:
                    diesel::JoinTo<table>,
                F: diesel::query_source::QuerySource,
            {
                type FromClause = diesel::internal::table_macro::SelectStatement<diesel::internal::table_macro::FromClause<F>, S, D, W, O, L, Of, G>;
                type OnClause = <diesel::internal::table_macro::SelectStatement<
                    diesel::internal::table_macro::FromClause<F>,
                    S,
                    D,
                    W,
                    O,
                    L,
                    Of,
                    G,
                > as diesel::JoinTo<table>>::OnClause;
                fn join_target(
                    __diesel_internal_rhs: diesel::internal::table_macro::SelectStatement<
                        diesel::internal::table_macro::FromClause<F>,
                        S,
                        D,
                        W,
                        O,
                        L,
                        Of,
                        G,
                    >,
                ) -> (Self::FromClause, Self::OnClause) {
                    let (_, __diesel_internal_on_clause) = diesel::internal::table_macro::SelectStatement::join_target(table);
                    (__diesel_internal_rhs, __diesel_internal_on_clause)
                }
            }
            impl<'a, QS, ST, DB>
                diesel::JoinTo<diesel::internal::table_macro::BoxedSelectStatement<'a, diesel::internal::table_macro::FromClause<QS>, ST, DB>>
                for table
            where
                diesel::internal::table_macro::BoxedSelectStatement<'a, diesel::internal::table_macro::FromClause<QS>, ST, DB>: diesel::JoinTo<table>,
                QS: diesel::query_source::QuerySource,
            {
                type FromClause = diesel::internal::table_macro::BoxedSelectStatement<'a, diesel::internal::table_macro::FromClause<QS>, ST, DB>;
                type OnClause = <diesel::internal::table_macro::BoxedSelectStatement<
                    'a,
                    diesel::internal::table_macro::FromClause<QS>,
                    ST,
                    DB,
                > as diesel::JoinTo<table>>::OnClause;
                fn join_target(
                    __diesel_internal_rhs: diesel::internal::table_macro::BoxedSelectStatement<
                        'a,
                        diesel::internal::table_macro::FromClause<QS>,
                        ST,
                        DB,
                    >,
                ) -> (Self::FromClause, Self::OnClause) {
                    let (_, __diesel_internal_on_clause) = diesel::internal::table_macro::BoxedSelectStatement::join_target(table);
                    (__diesel_internal_rhs, __diesel_internal_on_clause)
                }
            }
            impl<S> diesel::JoinTo<diesel::query_source::Alias<S>> for table
            where
                diesel::query_source::Alias<S>: diesel::JoinTo<table>,
            {
                type FromClause = diesel::query_source::Alias<S>;
                type OnClause = <diesel::query_source::Alias<S> as diesel::JoinTo<table>>::OnClause;
                fn join_target(__diesel_internal_rhs: diesel::query_source::Alias<S>) -> (Self::FromClause, Self::OnClause) {
                    let (_, __diesel_internal_on_clause) = diesel::query_source::Alias::<S>::join_target(table);
                    (__diesel_internal_rhs, __diesel_internal_on_clause)
                }
            }
            impl<T> diesel::insertable::Insertable<T> for table
            where
                <table as diesel::query_builder::AsQuery>::Query: diesel::insertable::Insertable<T>,
            {
                type Values = <<table as diesel::query_builder::AsQuery>::Query as diesel::insertable::Insertable<T>>::Values;
                fn values(self) -> Self::Values {
                    use diesel::query_builder::AsQuery;
                    self.as_query().values()
                }
            }
            impl<'a, T> diesel::insertable::Insertable<T> for &'a table
            where
                table: diesel::insertable::Insertable<T>,
            {
                type Values = <table as diesel::insertable::Insertable<T>>::Values;
                fn values(self) -> Self::Values {
                    (*self).values()
                }
            }
            /// Contains all of the columns of this table
            pub mod columns {
                use super::table;
                use ::diesel;
                use diesel::sql_types::*;
                #[allow(non_camel_case_types, dead_code)]
                /// Represents `table_name.*`, which is sometimes needed for
                /// efficient count queries. It cannot be used in place of
                /// `all_columns`, and has a `SqlType` of `()` to prevent it
                /// being used that way
                pub struct star;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for star {
                    #[inline]
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "star")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for star {
                    #[inline]
                    fn clone(&self) -> star {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for star {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for star {
                        type QueryId = star;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                impl<__GB> diesel::expression::ValidGrouping<__GB> for star
                where
                    (id, name, created_at, updated_at): diesel::expression::ValidGrouping<__GB>,
                {
                    type IsAggregate = <(id, name, created_at, updated_at) as diesel::expression::ValidGrouping<__GB>>::IsAggregate;
                }
                impl diesel::Expression for star {
                    type SqlType = diesel::expression::expression_types::NotSelectable;
                }
                impl<DB: diesel::backend::Backend> diesel::query_builder::QueryFragment<DB> for star
                where
                    <table as diesel::QuerySource>::FromClause: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<'_, 'b, DB>,
                    ) -> diesel::result::QueryResult<()> {
                        use diesel::QuerySource;
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<table> =
                                diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_sql("*");
                        Ok(())
                    }
                }
                impl diesel::SelectableExpression<table> for star {}
                impl diesel::AppearsOnTable<table> for star {}
                #[allow(non_camel_case_types, dead_code)]
                pub struct id;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for id {
                    #[inline]
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "id")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for id {
                    #[inline]
                    fn clone(&self) -> id {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for id {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for id {
                        type QueryId = id;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::default::Default for id {
                    #[inline]
                    fn default() -> id {
                        id {}
                    }
                }
                impl diesel::expression::Expression for id {
                    type SqlType = Unsigned<Integer>;
                }
                impl<DB> diesel::query_builder::QueryFragment<DB> for id
                where
                    DB: diesel::backend::Backend,
                    diesel::internal::table_macro::StaticQueryFragmentInstance<table>: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<'_, 'b, DB>,
                    ) -> diesel::result::QueryResult<()> {
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<table> =
                                diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_identifier("id")
                    }
                }
                impl diesel::SelectableExpression<super::table> for id {}
                impl<QS> diesel::AppearsOnTable<QS> for id where QS: diesel::query_source::AppearsInFromClause<super::table, Count = diesel::query_source::Once> {}
                impl<Left, Right>
                    diesel::SelectableExpression<diesel::internal::table_macro::Join<Left, Right, diesel::internal::table_macro::LeftOuter>> for id
                where
                    id: diesel::AppearsOnTable<diesel::internal::table_macro::Join<Left, Right, diesel::internal::table_macro::LeftOuter>>,
                    Self: diesel::SelectableExpression<Left>,
                    Right: diesel::query_source::AppearsInFromClause<super::table, Count = diesel::query_source::Never>
                        + diesel::query_source::QuerySource,
                    Left: diesel::query_source::QuerySource,
                {
                }
                impl<Left, Right> diesel::SelectableExpression<diesel::internal::table_macro::Join<Left, Right, diesel::internal::table_macro::Inner>> for id
                where
                    id: diesel::AppearsOnTable<diesel::internal::table_macro::Join<Left, Right, diesel::internal::table_macro::Inner>>,
                    Left: diesel::query_source::AppearsInFromClause<super::table> + diesel::query_source::QuerySource,
                    Right: diesel::query_source::AppearsInFromClause<super::table> + diesel::query_source::QuerySource,
                    (Left::Count, Right::Count): diesel::internal::table_macro::Pick<Left, Right>,
                    Self: diesel::SelectableExpression<<(Left::Count, Right::Count) as diesel::internal::table_macro::Pick<Left, Right>>::Selection>,
                {
                }
                impl<Join, On> diesel::SelectableExpression<diesel::internal::table_macro::JoinOn<Join, On>> for id where
                    id: diesel::SelectableExpression<Join> + diesel::AppearsOnTable<diesel::internal::table_macro::JoinOn<Join, On>>
                {
                }
                impl<From>
                    diesel::SelectableExpression<diesel::internal::table_macro::SelectStatement<diesel::internal::table_macro::FromClause<From>>>
                    for id
                where
                    From: diesel::query_source::QuerySource,
                    id: diesel::SelectableExpression<From>
                        + diesel::AppearsOnTable<diesel::internal::table_macro::SelectStatement<diesel::internal::table_macro::FromClause<From>>>,
                {
                }
                impl<__GB> diesel::expression::ValidGrouping<__GB> for id
                where
                    __GB: diesel::expression::IsContainedInGroupBy<id, Output = diesel::expression::is_contained_in_group_by::Yes>,
                {
                    type IsAggregate = diesel::expression::is_aggregate::Yes;
                }
                impl diesel::expression::ValidGrouping<()> for id {
                    type IsAggregate = diesel::expression::is_aggregate::No;
                }
                impl diesel::expression::IsContainedInGroupBy<id> for id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::query_source::Column for id {
                    type Table = super::table;
                    const NAME: &'static str = "id";
                }
                impl<T> diesel::EqAll<T> for id
                where
                    T: diesel::expression::AsExpression<Unsigned<Integer>>,
                    diesel::dsl::Eq<id, T::Expression>: diesel::Expression<SqlType = diesel::sql_types::Bool>,
                {
                    type Output = diesel::dsl::Eq<Self, T::Expression>;
                    fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                        use diesel::expression_methods::ExpressionMethods;
                        self.eq(__diesel_internal_rhs)
                    }
                }
                impl<Rhs> ::std::ops::Add<Rhs> for id
                where
                    Rhs: diesel::expression::AsExpression<<<id as diesel::Expression>::SqlType as diesel::sql_types::ops::Add>::Rhs>,
                {
                    type Output = diesel::internal::table_macro::ops::Add<Self, Rhs::Expression>;
                    fn add(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Add::new(self, __diesel_internal_rhs.as_expression())
                    }
                }
                impl<Rhs> ::std::ops::Sub<Rhs> for id
                where
                    Rhs: diesel::expression::AsExpression<<<id as diesel::Expression>::SqlType as diesel::sql_types::ops::Sub>::Rhs>,
                {
                    type Output = diesel::internal::table_macro::ops::Sub<Self, Rhs::Expression>;
                    fn sub(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Sub::new(self, __diesel_internal_rhs.as_expression())
                    }
                }
                impl<Rhs> ::std::ops::Div<Rhs> for id
                where
                    Rhs: diesel::expression::AsExpression<<<id as diesel::Expression>::SqlType as diesel::sql_types::ops::Div>::Rhs>,
                {
                    type Output = diesel::internal::table_macro::ops::Div<Self, Rhs::Expression>;
                    fn div(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Div::new(self, __diesel_internal_rhs.as_expression())
                    }
                }
                impl<Rhs> ::std::ops::Mul<Rhs> for id
                where
                    Rhs: diesel::expression::AsExpression<<<id as diesel::Expression>::SqlType as diesel::sql_types::ops::Mul>::Rhs>,
                {
                    type Output = diesel::internal::table_macro::ops::Mul<Self, Rhs::Expression>;
                    fn mul(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Mul::new(self, __diesel_internal_rhs.as_expression())
                    }
                }
                #[allow(non_camel_case_types, dead_code)]
                pub struct name;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for name {
                    #[inline]
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "name")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for name {
                    #[inline]
                    fn clone(&self) -> name {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for name {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for name {
                        type QueryId = name;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::default::Default for name {
                    #[inline]
                    fn default() -> name {
                        name {}
                    }
                }
                impl diesel::expression::Expression for name {
                    type SqlType = Varchar;
                }
                impl<DB> diesel::query_builder::QueryFragment<DB> for name
                where
                    DB: diesel::backend::Backend,
                    diesel::internal::table_macro::StaticQueryFragmentInstance<table>: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<'_, 'b, DB>,
                    ) -> diesel::result::QueryResult<()> {
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<table> =
                                diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_identifier("name")
                    }
                }
                impl diesel::SelectableExpression<super::table> for name {}
                impl<QS> diesel::AppearsOnTable<QS> for name where QS: diesel::query_source::AppearsInFromClause<super::table, Count = diesel::query_source::Once> {}
                impl<Left, Right>
                    diesel::SelectableExpression<diesel::internal::table_macro::Join<Left, Right, diesel::internal::table_macro::LeftOuter>> for name
                where
                    name: diesel::AppearsOnTable<diesel::internal::table_macro::Join<Left, Right, diesel::internal::table_macro::LeftOuter>>,
                    Self: diesel::SelectableExpression<Left>,
                    Right: diesel::query_source::AppearsInFromClause<super::table, Count = diesel::query_source::Never>
                        + diesel::query_source::QuerySource,
                    Left: diesel::query_source::QuerySource,
                {
                }
                impl<Left, Right> diesel::SelectableExpression<diesel::internal::table_macro::Join<Left, Right, diesel::internal::table_macro::Inner>> for name
                where
                    name: diesel::AppearsOnTable<diesel::internal::table_macro::Join<Left, Right, diesel::internal::table_macro::Inner>>,
                    Left: diesel::query_source::AppearsInFromClause<super::table> + diesel::query_source::QuerySource,
                    Right: diesel::query_source::AppearsInFromClause<super::table> + diesel::query_source::QuerySource,
                    (Left::Count, Right::Count): diesel::internal::table_macro::Pick<Left, Right>,
                    Self: diesel::SelectableExpression<<(Left::Count, Right::Count) as diesel::internal::table_macro::Pick<Left, Right>>::Selection>,
                {
                }
                impl<Join, On> diesel::SelectableExpression<diesel::internal::table_macro::JoinOn<Join, On>> for name where
                    name: diesel::SelectableExpression<Join> + diesel::AppearsOnTable<diesel::internal::table_macro::JoinOn<Join, On>>
                {
                }
                impl<From>
                    diesel::SelectableExpression<diesel::internal::table_macro::SelectStatement<diesel::internal::table_macro::FromClause<From>>>
                    for name
                where
                    From: diesel::query_source::QuerySource,
                    name: diesel::SelectableExpression<From>
                        + diesel::AppearsOnTable<diesel::internal::table_macro::SelectStatement<diesel::internal::table_macro::FromClause<From>>>,
                {
                }
                impl<__GB> diesel::expression::ValidGrouping<__GB> for name
                where
                    __GB: diesel::expression::IsContainedInGroupBy<name, Output = diesel::expression::is_contained_in_group_by::Yes>,
                {
                    type IsAggregate = diesel::expression::is_aggregate::Yes;
                }
                impl diesel::expression::ValidGrouping<()> for name {
                    type IsAggregate = diesel::expression::is_aggregate::No;
                }
                impl diesel::expression::IsContainedInGroupBy<name> for name {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::query_source::Column for name {
                    type Table = super::table;
                    const NAME: &'static str = "name";
                }
                impl<T> diesel::EqAll<T> for name
                where
                    T: diesel::expression::AsExpression<Varchar>,
                    diesel::dsl::Eq<name, T::Expression>: diesel::Expression<SqlType = diesel::sql_types::Bool>,
                {
                    type Output = diesel::dsl::Eq<Self, T::Expression>;
                    fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                        use diesel::expression_methods::ExpressionMethods;
                        self.eq(__diesel_internal_rhs)
                    }
                }
                impl self::diesel::query_source::SizeRestrictedColumn for name {
                    const MAX_LENGTH: usize = 255;
                }
                #[allow(non_camel_case_types, dead_code)]
                pub struct created_at;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for created_at {
                    #[inline]
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "created_at")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for created_at {
                    #[inline]
                    fn clone(&self) -> created_at {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for created_at {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for created_at {
                        type QueryId = created_at;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::default::Default for created_at {
                    #[inline]
                    fn default() -> created_at {
                        created_at {}
                    }
                }
                impl diesel::expression::Expression for created_at {
                    type SqlType = Timestamp;
                }
                impl<DB> diesel::query_builder::QueryFragment<DB> for created_at
                where
                    DB: diesel::backend::Backend,
                    diesel::internal::table_macro::StaticQueryFragmentInstance<table>: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<'_, 'b, DB>,
                    ) -> diesel::result::QueryResult<()> {
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<table> =
                                diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_identifier("created_at")
                    }
                }
                impl diesel::SelectableExpression<super::table> for created_at {}
                impl<QS> diesel::AppearsOnTable<QS> for created_at where
                    QS: diesel::query_source::AppearsInFromClause<super::table, Count = diesel::query_source::Once>
                {
                }
                impl<Left, Right>
                    diesel::SelectableExpression<diesel::internal::table_macro::Join<Left, Right, diesel::internal::table_macro::LeftOuter>>
                    for created_at
                where
                    created_at: diesel::AppearsOnTable<diesel::internal::table_macro::Join<Left, Right, diesel::internal::table_macro::LeftOuter>>,
                    Self: diesel::SelectableExpression<Left>,
                    Right: diesel::query_source::AppearsInFromClause<super::table, Count = diesel::query_source::Never>
                        + diesel::query_source::QuerySource,
                    Left: diesel::query_source::QuerySource,
                {
                }
                impl<Left, Right> diesel::SelectableExpression<diesel::internal::table_macro::Join<Left, Right, diesel::internal::table_macro::Inner>> for created_at
                where
                    created_at: diesel::AppearsOnTable<diesel::internal::table_macro::Join<Left, Right, diesel::internal::table_macro::Inner>>,
                    Left: diesel::query_source::AppearsInFromClause<super::table> + diesel::query_source::QuerySource,
                    Right: diesel::query_source::AppearsInFromClause<super::table> + diesel::query_source::QuerySource,
                    (Left::Count, Right::Count): diesel::internal::table_macro::Pick<Left, Right>,
                    Self: diesel::SelectableExpression<<(Left::Count, Right::Count) as diesel::internal::table_macro::Pick<Left, Right>>::Selection>,
                {
                }
                impl<Join, On> diesel::SelectableExpression<diesel::internal::table_macro::JoinOn<Join, On>> for created_at where
                    created_at: diesel::SelectableExpression<Join> + diesel::AppearsOnTable<diesel::internal::table_macro::JoinOn<Join, On>>
                {
                }
                impl<From>
                    diesel::SelectableExpression<diesel::internal::table_macro::SelectStatement<diesel::internal::table_macro::FromClause<From>>>
                    for created_at
                where
                    From: diesel::query_source::QuerySource,
                    created_at: diesel::SelectableExpression<From>
                        + diesel::AppearsOnTable<diesel::internal::table_macro::SelectStatement<diesel::internal::table_macro::FromClause<From>>>,
                {
                }
                impl<__GB> diesel::expression::ValidGrouping<__GB> for created_at
                where
                    __GB: diesel::expression::IsContainedInGroupBy<created_at, Output = diesel::expression::is_contained_in_group_by::Yes>,
                {
                    type IsAggregate = diesel::expression::is_aggregate::Yes;
                }
                impl diesel::expression::ValidGrouping<()> for created_at {
                    type IsAggregate = diesel::expression::is_aggregate::No;
                }
                impl diesel::expression::IsContainedInGroupBy<created_at> for created_at {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::query_source::Column for created_at {
                    type Table = super::table;
                    const NAME: &'static str = "created_at";
                }
                impl<T> diesel::EqAll<T> for created_at
                where
                    T: diesel::expression::AsExpression<Timestamp>,
                    diesel::dsl::Eq<created_at, T::Expression>: diesel::Expression<SqlType = diesel::sql_types::Bool>,
                {
                    type Output = diesel::dsl::Eq<Self, T::Expression>;
                    fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                        use diesel::expression_methods::ExpressionMethods;
                        self.eq(__diesel_internal_rhs)
                    }
                }
                impl<Rhs> ::std::ops::Add<Rhs> for created_at
                where
                    Rhs: diesel::expression::AsExpression<<<created_at as diesel::Expression>::SqlType as diesel::sql_types::ops::Add>::Rhs>,
                {
                    type Output = diesel::internal::table_macro::ops::Add<Self, Rhs::Expression>;
                    fn add(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Add::new(self, __diesel_internal_rhs.as_expression())
                    }
                }
                impl<Rhs> ::std::ops::Sub<Rhs> for created_at
                where
                    Rhs: diesel::expression::AsExpression<<<created_at as diesel::Expression>::SqlType as diesel::sql_types::ops::Sub>::Rhs>,
                {
                    type Output = diesel::internal::table_macro::ops::Sub<Self, Rhs::Expression>;
                    fn sub(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Sub::new(self, __diesel_internal_rhs.as_expression())
                    }
                }
                #[allow(non_camel_case_types, dead_code)]
                pub struct updated_at;
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::fmt::Debug for updated_at {
                    #[inline]
                    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(f, "updated_at")
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::clone::Clone for updated_at {
                    #[inline]
                    fn clone(&self) -> updated_at {
                        *self
                    }
                }
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::marker::Copy for updated_at {}
                #[allow(unused_imports)]
                const _: () = {
                    use diesel;
                    use diesel::query_builder::QueryId;
                    #[allow(non_camel_case_types)]
                    impl QueryId for updated_at {
                        type QueryId = updated_at;
                        const HAS_STATIC_QUERY_ID: bool = true;
                    }
                };
                #[automatically_derived]
                #[allow(non_camel_case_types, dead_code)]
                impl ::core::default::Default for updated_at {
                    #[inline]
                    fn default() -> updated_at {
                        updated_at {}
                    }
                }
                impl diesel::expression::Expression for updated_at {
                    type SqlType = Timestamp;
                }
                impl<DB> diesel::query_builder::QueryFragment<DB> for updated_at
                where
                    DB: diesel::backend::Backend,
                    diesel::internal::table_macro::StaticQueryFragmentInstance<table>: diesel::query_builder::QueryFragment<DB>,
                {
                    #[allow(non_snake_case)]
                    fn walk_ast<'b>(
                        &'b self,
                        mut __diesel_internal_out: diesel::query_builder::AstPass<'_, 'b, DB>,
                    ) -> diesel::result::QueryResult<()> {
                        if !__diesel_internal_out.should_skip_from() {
                            const FROM_CLAUSE: diesel::internal::table_macro::StaticQueryFragmentInstance<table> =
                                diesel::internal::table_macro::StaticQueryFragmentInstance::new();
                            FROM_CLAUSE.walk_ast(__diesel_internal_out.reborrow())?;
                            __diesel_internal_out.push_sql(".");
                        }
                        __diesel_internal_out.push_identifier("updated_at")
                    }
                }
                impl diesel::SelectableExpression<super::table> for updated_at {}
                impl<QS> diesel::AppearsOnTable<QS> for updated_at where
                    QS: diesel::query_source::AppearsInFromClause<super::table, Count = diesel::query_source::Once>
                {
                }
                impl<Left, Right>
                    diesel::SelectableExpression<diesel::internal::table_macro::Join<Left, Right, diesel::internal::table_macro::LeftOuter>>
                    for updated_at
                where
                    updated_at: diesel::AppearsOnTable<diesel::internal::table_macro::Join<Left, Right, diesel::internal::table_macro::LeftOuter>>,
                    Self: diesel::SelectableExpression<Left>,
                    Right: diesel::query_source::AppearsInFromClause<super::table, Count = diesel::query_source::Never>
                        + diesel::query_source::QuerySource,
                    Left: diesel::query_source::QuerySource,
                {
                }
                impl<Left, Right> diesel::SelectableExpression<diesel::internal::table_macro::Join<Left, Right, diesel::internal::table_macro::Inner>> for updated_at
                where
                    updated_at: diesel::AppearsOnTable<diesel::internal::table_macro::Join<Left, Right, diesel::internal::table_macro::Inner>>,
                    Left: diesel::query_source::AppearsInFromClause<super::table> + diesel::query_source::QuerySource,
                    Right: diesel::query_source::AppearsInFromClause<super::table> + diesel::query_source::QuerySource,
                    (Left::Count, Right::Count): diesel::internal::table_macro::Pick<Left, Right>,
                    Self: diesel::SelectableExpression<<(Left::Count, Right::Count) as diesel::internal::table_macro::Pick<Left, Right>>::Selection>,
                {
                }
                impl<Join, On> diesel::SelectableExpression<diesel::internal::table_macro::JoinOn<Join, On>> for updated_at where
                    updated_at: diesel::SelectableExpression<Join> + diesel::AppearsOnTable<diesel::internal::table_macro::JoinOn<Join, On>>
                {
                }
                impl<From>
                    diesel::SelectableExpression<diesel::internal::table_macro::SelectStatement<diesel::internal::table_macro::FromClause<From>>>
                    for updated_at
                where
                    From: diesel::query_source::QuerySource,
                    updated_at: diesel::SelectableExpression<From>
                        + diesel::AppearsOnTable<diesel::internal::table_macro::SelectStatement<diesel::internal::table_macro::FromClause<From>>>,
                {
                }
                impl<__GB> diesel::expression::ValidGrouping<__GB> for updated_at
                where
                    __GB: diesel::expression::IsContainedInGroupBy<updated_at, Output = diesel::expression::is_contained_in_group_by::Yes>,
                {
                    type IsAggregate = diesel::expression::is_aggregate::Yes;
                }
                impl diesel::expression::ValidGrouping<()> for updated_at {
                    type IsAggregate = diesel::expression::is_aggregate::No;
                }
                impl diesel::expression::IsContainedInGroupBy<updated_at> for updated_at {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::query_source::Column for updated_at {
                    type Table = super::table;
                    const NAME: &'static str = "updated_at";
                }
                impl<T> diesel::EqAll<T> for updated_at
                where
                    T: diesel::expression::AsExpression<Timestamp>,
                    diesel::dsl::Eq<updated_at, T::Expression>: diesel::Expression<SqlType = diesel::sql_types::Bool>,
                {
                    type Output = diesel::dsl::Eq<Self, T::Expression>;
                    fn eq_all(self, __diesel_internal_rhs: T) -> Self::Output {
                        use diesel::expression_methods::ExpressionMethods;
                        self.eq(__diesel_internal_rhs)
                    }
                }
                impl<Rhs> ::std::ops::Add<Rhs> for updated_at
                where
                    Rhs: diesel::expression::AsExpression<<<updated_at as diesel::Expression>::SqlType as diesel::sql_types::ops::Add>::Rhs>,
                {
                    type Output = diesel::internal::table_macro::ops::Add<Self, Rhs::Expression>;
                    fn add(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Add::new(self, __diesel_internal_rhs.as_expression())
                    }
                }
                impl<Rhs> ::std::ops::Sub<Rhs> for updated_at
                where
                    Rhs: diesel::expression::AsExpression<<<updated_at as diesel::Expression>::SqlType as diesel::sql_types::ops::Sub>::Rhs>,
                {
                    type Output = diesel::internal::table_macro::ops::Sub<Self, Rhs::Expression>;
                    fn sub(self, __diesel_internal_rhs: Rhs) -> Self::Output {
                        diesel::internal::table_macro::ops::Sub::new(self, __diesel_internal_rhs.as_expression())
                    }
                }
                impl diesel::expression::IsContainedInGroupBy<id> for name {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<name> for id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::expression::IsContainedInGroupBy<id> for created_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<created_at> for id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::expression::IsContainedInGroupBy<id> for updated_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<updated_at> for id {
                    type Output = diesel::expression::is_contained_in_group_by::Yes;
                }
                impl diesel::expression::IsContainedInGroupBy<name> for created_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<created_at> for name {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<name> for updated_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<updated_at> for name {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<created_at> for updated_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
                impl diesel::expression::IsContainedInGroupBy<updated_at> for created_at {
                    type Output = diesel::expression::is_contained_in_group_by::No;
                }
            }
        }
    }
}
mod modules {
    pub mod user {
        use nidrs::module;
        pub mod controller {
            use super::{dto::CreateUserDto, service::UserService};
            use crate::models::entities::user::User;
            use axum::{
                extract::{Path, Query},
                Json,
            };
            use nidrs::externs::axum;
            use nidrs::openapi::api;
            use nidrs::{post, AppResult, Inject};
            use nidrs_macro::{controller, get};
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
                    meta.set("service", "UserController");
                    meta.set_data(nidrs::datasets::ControllerPath::from("/user"));
                    meta.set_data(nidrs::datasets::ServiceName::from("UserController"));
                    meta.set("module", "UserModule");
                    meta.set("global", "app");
                    meta
                }
            }
            impl UserController {
                pub async fn get_user_all(&self, Query(q): Query<HashMap<String, String>>) -> AppResult<Json<Vec<User>>> {
                    {
                        ::std::io::_print(format_args!("Query {0:?}\n", q));
                    };
                    Ok(Json(self.user_service.all().await?))
                }
                pub fn __meta_get_user_all(&self) -> nidrs::InnerMeta {
                    let mut meta = nidrs::InnerMeta::new();
                    meta.set("handler", "get_user_all");
                    meta.set_data(nidrs::datasets::RouterName::from("get_user_all"));
                    meta.set_data(nidrs::datasets::RouterPath::from("/"));
                    meta.set_data(nidrs::datasets::RouterMethod::from("get"));
                    meta.set_data(nidrs::datasets::ServiceType::from("Controller"));
                    meta.set("service", "UserController");
                    meta.set_data(nidrs::datasets::ControllerPath::from("/user"));
                    meta.set_data(nidrs::datasets::ServiceName::from("UserController"));
                    meta.set("module", "UserModule");
                    meta.set("global", "app");
                    meta
                }
                pub fn __route_get_user_all(&self, mut ctx: nidrs::ModuleCtx, module: &str) -> nidrs::ModuleCtx {
                    use axum::response::IntoResponse;
                    use nidrs::externs::axum;
                    use nidrs::externs::axum::{extract::Query, Json};
                    use nidrs::externs::meta::{InnerMeta, Meta};
                    use nidrs::externs::serde_json::Value;
                    use nidrs::Interceptor;
                    let mut meta = self.__meta_get_user_all();
                    let router_info = ctx.get_router_full(&meta);
                    if let Err(e) = router_info {
                        {
                            ::core::panicking::panic_fmt(format_args!("[{0}] {1:?}", "__route_get_user_all", e));
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
                                let r = t_controller.get_user_all(p0).await;
                                r
                            }),
                        )
                        .route_layer(nidrs::externs::axum::Extension(meta.clone()));
                    ctx.routers.push(nidrs::MetaRouter::new(router, meta));
                    ctx
                }
                pub async fn get_user_by_id(&self, Path(user_id): Path<u32>, Query(q): Query<HashMap<String, String>>) -> AppResult<Json<User>> {
                    {
                        ::std::io::_print(format_args!("Query {0:?}\n", q));
                    };
                    Ok(Json(self.user_service.find_by_id(user_id).await?))
                }
                pub fn __meta_get_user_by_id(&self) -> nidrs::InnerMeta {
                    let mut meta = nidrs::InnerMeta::new();
                    meta.set_data(nidrs::datasets::RouterPath::from("/:id"));
                    meta.set_data(nidrs::datasets::RouterName::from("get_user_by_id"));
                    meta.set_data(nidrs::datasets::RouterMethod::from("get"));
                    meta.set("handler", "get_user_by_id");
                    meta.set_data(nidrs::datasets::ServiceType::from("Controller"));
                    meta.set("service", "UserController");
                    meta.set_data(nidrs::datasets::ControllerPath::from("/user"));
                    meta.set_data(nidrs::datasets::ServiceName::from("UserController"));
                    meta.set("module", "UserModule");
                    meta.set("global", "app");
                    meta
                }
                pub fn __route_get_user_by_id(&self, mut ctx: nidrs::ModuleCtx, module: &str) -> nidrs::ModuleCtx {
                    use axum::response::IntoResponse;
                    use nidrs::externs::axum;
                    use nidrs::externs::axum::{extract::Query, Json};
                    use nidrs::externs::meta::{InnerMeta, Meta};
                    use nidrs::externs::serde_json::Value;
                    use nidrs::Interceptor;
                    let mut meta = self.__meta_get_user_by_id();
                    let router_info = ctx.get_router_full(&meta);
                    if let Err(e) = router_info {
                        {
                            ::core::panicking::panic_fmt(format_args!("[{0}] {1:?}", "__route_get_user_by_id", e));
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
                                let r = t_controller.get_user_by_id(p0, p1).await;
                                r
                            }),
                        )
                        .route_layer(nidrs::externs::axum::Extension(meta.clone()));
                    ctx.routers.push(nidrs::MetaRouter::new(router, meta));
                    ctx
                }
                pub async fn create_user(&self, Json(j): Json<CreateUserDto>) -> AppResult<String> {
                    {
                        ::std::io::_print(format_args!("Query {0:?}\n", j));
                    };
                    Ok(self.user_service.create(j).await?.to_string())
                }
                pub fn __meta_create_user(&self) -> nidrs::InnerMeta {
                    let mut meta = nidrs::InnerMeta::new();
                    meta.set_data(nidrs::datasets::RouterPath::from("/"));
                    meta.set("handler", "create_user");
                    meta.set_data(nidrs::openapi::RouterOut(nidrs::openapi::RouterParams::default().comb::<AppResult<String>>("")));
                    meta.set_data(nidrs::openapi::RouterIn(nidrs::openapi::RouterParams::default().comb::<Json<CreateUserDto>>("Json(j)")));
                    meta.set_data(nidrs::datasets::RouterName::from("create_user"));
                    meta.set_data(nidrs::datasets::RouterMethod::from("post"));
                    meta.set_data(nidrs::datasets::ServiceType::from("Controller"));
                    meta.set("service", "UserController");
                    meta.set_data(nidrs::datasets::ControllerPath::from("/user"));
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
                    use nidrs::externs::serde_json::Value;
                    use nidrs::Interceptor;
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
            use nidrs::externs::serde;
            use nidrs::openapi::schema;
            use serde::Deserialize;
            pub struct CreateUserDto {
                pub name: String,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for CreateUserDto {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field1_finish(f, "CreateUserDto", "name", &&self.name)
                }
            }
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
                                    "name" => _serde::__private::Ok(__Field::__field0),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(self, __value: &[u8]) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"name" => _serde::__private::Ok(__Field::__field0),
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
                                            &"struct CreateUserDto with 1 element",
                                        ));
                                    }
                                };
                                _serde::__private::Ok(CreateUserDto { name: __field0 })
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
                                                return _serde::__private::Err(<__A::Error as _serde::de::Error>::duplicate_field("name"));
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
                                    _serde::__private::None => _serde::__private::de::missing_field("name")?,
                                };
                                _serde::__private::Ok(CreateUserDto { name: __field0 })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["name"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "CreateUserDto",
                            FIELDS,
                            __Visitor { marker: _serde::__private::PhantomData::<CreateUserDto>, lifetime: _serde::__private::PhantomData },
                        )
                    }
                }
            };
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
                    match dto_type {
                        nidrs::openapi::ParamDtoIn::Body => {
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
                            nidrs::openapi::ParamDto::BodySchema((ref_schema, schemas))
                        }
                        _ => nidrs::openapi::ParamDto::None,
                    }
                }
            }
        }
        pub mod service {
            use super::dto::CreateUserDto;
            use crate::models::entities::user::{User, UserEntity};
            use nidrs::{AppResult, Inject};
            use nidrs_macro::injectable;
            pub struct UserService {
                user_entity: Inject<UserEntity>,
            }
            #[automatically_derived]
            impl ::core::default::Default for UserService {
                #[inline]
                fn default() -> UserService {
                    UserService { user_entity: ::core::default::Default::default() }
                }
            }
            impl nidrs::Service for UserService {
                fn inject(&self, ctx: nidrs::ModuleCtx, module_name: &str) -> nidrs::ModuleCtx {
                    let service = ctx.get_service::<UserEntity>(&module_name, "UserEntity");
                    self.user_entity.inject(service.clone());
                    ctx
                }
            }
            impl nidrs::ImplMeta for UserService {
                fn __meta(&self) -> nidrs::InnerMeta {
                    let mut meta = nidrs::InnerMeta::new();
                    meta.set_data(nidrs::datasets::ServiceName::from("UserService"));
                    meta.set("service", "UserService");
                    meta.set_data(nidrs::datasets::ServiceType::from("Service"));
                    meta.set("module", "UserModule");
                    meta.set("global", "app");
                    meta
                }
            }
            impl UserService {
                pub async fn create(&self, part: CreateUserDto) -> AppResult<usize> {
                    self.user_entity.create(part.name).await
                }
                pub async fn find_by_id(&self, id: u32) -> AppResult<User> {
                    self.user_entity.find_by_id(id).await
                }
                pub async fn all(&self) -> AppResult<Vec<User>> {
                    self.user_entity.all().await
                }
                pub async fn update_name_by_id(&self, id: u32, name: String) -> AppResult<usize> {
                    self.user_entity.update(id, name).await
                }
            }
        }
        use crate::models::entities::user::UserEntity;
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
                ctx.imports.insert("UserModule".to_string(), Vec::from([]));
                ctx.append_exports("UserModule", Vec::<&str>::from(["UserService"]), false);
                if ctx.register_controller("UserModule", "UserController", Box::new(std::sync::Arc::new(UserController::default()))) {
                    let t_controller = ctx.get_controller::<UserController>("UserModule", "UserController");
                    ctx = t_controller.__route_get_user_all(ctx, "UserModule");
                    ctx = t_controller.__route_get_user_by_id(ctx, "UserModule");
                    ctx = t_controller.__route_create_user(ctx, "UserModule");
                }
                let svc = std::sync::Arc::new(UserService::default());
                ctx.register_service("UserModule", "UserService", Box::new(svc));
                let svc = std::sync::Arc::new(UserEntity::default());
                ctx.register_service("UserModule", "UserEntity", Box::new(svc));
                let t = ctx.get_service::<UserService>("UserModule", "UserService");
                {
                    ::std::io::_print(format_args!("{0} ", nidrs_extern::colored::Colorize::green("[nidrs]"),));
                };
                {
                    ::std::io::_print(format_args!("Injecting {0}::{1}.\n", "UserModule", "UserService",));
                };
                let ctx = t.inject(ctx, &"UserModule");
                let t = ctx.get_service::<UserEntity>("UserModule", "UserEntity");
                {
                    ::std::io::_print(format_args!("{0} ", nidrs_extern::colored::Colorize::green("[nidrs]"),));
                };
                {
                    ::std::io::_print(format_args!("Injecting {0}::{1}.\n", "UserModule", "UserEntity"));
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
                meta.set("service", "UserModule");
                meta.set("__", true);
                meta.set("module", "UserModule");
                meta.set("global", "app");
                meta
            }
        }
    }
}
pub use nidrs::AppError;
pub use nidrs::AppResult;
fn main() {
    let app = nidrs::NidrsFactory::create(app::AppModule);
    let app = app.default_prefix("/api/{version}");
    let app = app.default_version("v1");
    app.listen(3000).block();
}
extern crate alloc;
