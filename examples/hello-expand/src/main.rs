#![feature(print_internals)]
#![feature(panic_internals)]
#![feature(alloc)]
#![feature(fmt_helpers_for_derive)]
#![allow(warnings, unused)]
// meta "AppController" ["version"]
// meta "AppController" ["role", "auth"]
// controller AppController []
// meta "AppController" ["METADATA:nidrs::datasets::ServiceType::Controller"]
// meta "AppController" ["METADATA:nidrs::datasets::ServiceName"]
// meta "AppController" ["METADATA:nidrs::datasets::ControllerPath"]
// service_derive "AppController"
// meta "get_hello_world" ["arr"]
// meta "get_hello_world" ["METADATA:nidrs::datasets::DisableDefaultPrefix"]
// meta "get_hello_world" ["version"]
// route get /hello None Some(String("v2"))
// meta "get_hello_world" ["METADATA:nidrs::datasets::RouterName"]
// meta "get_hello_world" ["METADATA:nidrs::datasets::RouterMethod"]
// meta "get_hello_world" ["METADATA:nidrs::datasets::RouterPath"]
// route_derive "get_hello_world"
// route get /hello2 None Some(String("v1"))
// meta "get_hello_world2" ["METADATA:nidrs::datasets::RouterName"]
// meta "get_hello_world2" ["METADATA:nidrs::datasets::RouterMethod"]
// meta "get_hello_world2" ["METADATA:nidrs::datasets::RouterPath"]
// route_derive "get_hello_world2"
// route post /hello None Some(String("v1"))
// meta "post_hello_world" ["METADATA:nidrs::datasets::RouterName"]
// meta "post_hello_world" ["METADATA:nidrs::datasets::RouterMethod"]
// meta "post_hello_world" ["METADATA:nidrs::datasets::RouterPath"]
// route_derive "post_hello_world"
// meta "AppService" ["METADATA:nidrs::datasets::ServiceType::Service"]
// meta "AppService" ["METADATA:nidrs::datasets::ServiceName"]
// service_derive "AppService"
// module "AppModule"
// meta "ConfOptions" ["METADATA:nidrs::datasets::ServiceType::Service"]
// meta "ConfOptions" ["METADATA:nidrs::datasets::ServiceName"]
// service_derive "ConfOptions"
// meta "ConfService" ["METADATA:nidrs::datasets::ServiceType::Service"]
// meta "ConfService" ["METADATA:nidrs::datasets::ServiceName"]
// service_derive "ConfService"
// meta "ConfModule" ["METADATA:nidrs::datasets::Global"]
// module "ConfModule"
// meta "LogInterceptor" ["METADATA:nidrs::datasets::ServiceType::Interceptor"]
// meta "LogInterceptor" ["METADATA:nidrs::datasets::ServiceName"]
// service_derive "LogInterceptor"
// meta "LogService" ["METADATA:nidrs::datasets::ServiceType::Service"]
// meta "LogService" ["METADATA:nidrs::datasets::ServiceName"]
// service_derive "LogService"
// meta "LogModule" ["METADATA:nidrs::datasets::Global"]
// module "LogModule"
// controller UserController []
// meta "UserController" ["METADATA:nidrs::datasets::ServiceType::Controller"]
// meta "UserController" ["METADATA:nidrs::datasets::ServiceName"]
// meta "UserController" ["METADATA:nidrs::datasets::ControllerPath"]
// service_derive "UserController"
// route get /hello None None
// meta "get_hello_world" ["METADATA:nidrs::datasets::RouterName"]
// meta "get_hello_world" ["METADATA:nidrs::datasets::RouterMethod"]
// meta "get_hello_world" ["METADATA:nidrs::datasets::RouterPath"]
// route_derive "get_hello_world"
// route post / None None
// meta "create_user" ["METADATA:nidrs::datasets::RouterName"]
// meta "create_user" ["METADATA:nidrs::datasets::RouterMethod"]
// meta "create_user" ["METADATA:nidrs::datasets::RouterPath"]
// route_derive "create_user"
// meta "UserService" ["METADATA:nidrs::datasets::ServiceType::Service"]
// meta "UserService" ["METADATA:nidrs::datasets::ServiceName"]
// service_derive "UserService"
// module "UserModule"
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod app {
    use nidrs::default_uses;
    use nidrs::macros::module;
    pub mod controller {
        use std::collections::HashMap;
        use nidrs::externs::axum::{extract::Query, response::AppendHeaders, Json};
        use nidrs::externs::serde_json;
        use nidrs::macros::{controller, get, meta, post};
        use nidrs::{version, Inject, Meta};
        use crate::AppResult;
        use super::{
            dto::{ArgDto, Status},
            service::AppService,
        };
        pub struct AppController {
            app_service: Inject<AppService>,
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
        impl nidrs::ControllerService for AppController {}
        impl nidrs::Service for AppController {
            fn inject(
                &self,
                ctx: nidrs::ModuleCtx,
                module_name: &str,
            ) -> nidrs::ModuleCtx {
                let service = ctx.get_service::<AppService>(&module_name, "AppService");
                self.app_service.inject(service.clone());
                ctx
            }
        }
        impl nidrs::ImplMeta for AppController {
            fn __meta() -> nidrs::Meta {
                let mut meta = nidrs::Meta::new();
                meta.set("version".to_string(), "v1");
                meta.set("role".to_string(), "admin");
                meta.set("auth".to_string(), "true");
                meta.set_data(nidrs::datasets::ServiceType::Controller);
                meta.set_data(nidrs::datasets::ServiceName("AppController"));
                meta.set_data(nidrs::datasets::ControllerPath(""));
                meta
            }
        }
        impl AppController {
            pub async fn get_hello_world(
                &self,
                meta: Meta,
                Query(q): Query<HashMap<String, String>>,
            ) -> AppResult<(AppendHeaders<[(String, String); 2]>, Status)> {
                {
                    ::std::io::_print(format_args!("Query {0:?}\n", q));
                };
                {
                    ::std::io::_print(format_args!("Meta Keys {0:?}\n", meta.keys()));
                };
                {
                    ::std::io::_print(
                        format_args!("Meta {0:?}\n", meta.get::<&str>("role")),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "Meta {0:?}\n",
                            meta.get_data::<nidrs::datasets::DisableDefaultPrefix>(),
                        ),
                    );
                };
                Ok((
                    AppendHeaders([
                        ("X-Custom-Header".to_string(), "hello".to_string()),
                        ("X-Custom-Header".to_string(), "world".to_string()),
                    ]),
                    Status {
                        db: "ok".to_string(),
                        redis: "ok".to_string(),
                    },
                ))
            }
            pub fn __meta_get_hello_world(&self) -> nidrs::Meta {
                let mut meta = nidrs::Meta::new();
                meta.set("arr".to_string(), Vec::from(["user"]));
                meta.set_data(nidrs::datasets::DisableDefaultPrefix(false));
                meta.set("version".to_string(), "v2");
                meta.set_data(nidrs::datasets::RouterName("get_hello_world"));
                meta.set_data(nidrs::datasets::RouterMethod("get"));
                meta.set_data(nidrs::datasets::RouterPath("/hello"));
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
                meta.set_data(nidrs::datasets::RouterName("get_hello_world2"));
                meta.set_data(nidrs::datasets::RouterMethod("get"));
                meta.set_data(nidrs::datasets::RouterPath("/hello2"));
                meta
            }
            pub async fn post_hello_world(
                &self,
                Query(q): Query<HashMap<String, String>>,
                Json(j): Json<ArgDto>,
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
                meta.set_data(nidrs::datasets::RouterName("post_hello_world"));
                meta.set_data(nidrs::datasets::RouterMethod("post"));
                meta.set_data(nidrs::datasets::RouterPath("/hello"));
                meta
            }
        }
    }
    pub mod dto {
        use nidrs::{
            externs::axum::{
                body::Body, http::{header, StatusCode},
                response::{IntoResponse, Response},
            },
            valid_macro::dto,
        };
        use nidrs::externs::serde::{Deserialize, Serialize};
        use nidrs::externs::serde_json;
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
        pub struct A {
            #[rule(Email)]
            pub hello: String,
            #[rule(Valid(v))]
            pub hello2: B,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for A {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "A",
                        false as usize + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "hello",
                        &self.hello,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "hello2",
                        &self.hello2,
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
            impl<'de> _serde::Deserialize<'de> for A {
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
                                "hello" => _serde::__private::Ok(__Field::__field0),
                                "hello2" => _serde::__private::Ok(__Field::__field1),
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
                                b"hello" => _serde::__private::Ok(__Field::__field0),
                                b"hello2" => _serde::__private::Ok(__Field::__field1),
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
                        marker: _serde::__private::PhantomData<A>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = A;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct A",
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
                                            &"struct A with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                B,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct A with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(A {
                                hello: __field0,
                                hello2: __field1,
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
                            let mut __field1: _serde::__private::Option<B> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("hello"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("hello2"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<B>(&mut __map)?,
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
                                    _serde::__private::de::missing_field("hello")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("hello2")?
                                }
                            };
                            _serde::__private::Ok(A {
                                hello: __field0,
                                hello2: __field1,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["hello", "hello2"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "A",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<A>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for A {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "A",
                    "hello",
                    &self.hello,
                    "hello2",
                    &&self.hello2,
                )
            }
        }
        impl nidrs::valid::validator::Validator for A {
            fn valid(&self) -> nidrs::valid::validator::ValidResult {
                use nidrs::valid::validator::Rule;
                use nidrs::valid::ruleset;
                use nidrs::valid::ruleset::*;
                let v = &self.hello;
                Email.valid(v, "hello", None)?;
                let v = &self.hello2;
                Valid(v).valid(v, "hello2", None)?;
                return Ok(());
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
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "B",
                        false as usize + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "hello2",
                        &self.hello2,
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
            impl<'de> _serde::Deserialize<'de> for B {
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
                                "hello2" => _serde::__private::Ok(__Field::__field0),
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
                                b"hello2" => _serde::__private::Ok(__Field::__field0),
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
                        marker: _serde::__private::PhantomData<B>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = B;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct B",
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
                                            &"struct B with 1 element",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(B { hello2: __field0 })
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
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("hello2"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
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
                                    _serde::__private::de::missing_field("hello2")?
                                }
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
                        __Visitor {
                            marker: _serde::__private::PhantomData::<B>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for B {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "B",
                    "hello2",
                    &&self.hello2,
                )
            }
        }
        impl nidrs::valid::validator::Validator for B {
            fn valid(&self) -> nidrs::valid::validator::ValidResult {
                use nidrs::valid::validator::Rule;
                use nidrs::valid::ruleset;
                use nidrs::valid::ruleset::*;
                return Ok(());
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
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    match *self {
                        ArgDto::A(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "ArgDto",
                                0u32,
                                "A",
                                __field0,
                            )
                        }
                        ArgDto::B(ref __field0) => {
                            _serde::Serializer::serialize_newtype_variant(
                                __serializer,
                                "ArgDto",
                                1u32,
                                "B",
                                __field0,
                            )
                        }
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
                                "variant identifier",
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
                                _ => {
                                    _serde::__private::Err(
                                        _serde::de::Error::invalid_value(
                                            _serde::de::Unexpected::Unsigned(__value),
                                            &"variant index 0 <= i < 2",
                                        ),
                                    )
                                }
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
                                "A" => _serde::__private::Ok(__Field::__field0),
                                "B" => _serde::__private::Ok(__Field::__field1),
                                _ => {
                                    _serde::__private::Err(
                                        _serde::de::Error::unknown_variant(__value, VARIANTS),
                                    )
                                }
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
                                b"A" => _serde::__private::Ok(__Field::__field0),
                                b"B" => _serde::__private::Ok(__Field::__field1),
                                _ => {
                                    let __value = &_serde::__private::from_utf8_lossy(__value);
                                    _serde::__private::Err(
                                        _serde::de::Error::unknown_variant(__value, VARIANTS),
                                    )
                                }
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
                        marker: _serde::__private::PhantomData<ArgDto>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = ArgDto;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "enum ArgDto",
                            )
                        }
                        fn visit_enum<__A>(
                            self,
                            __data: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::EnumAccess<'de>,
                        {
                            match _serde::de::EnumAccess::variant(__data)? {
                                (__Field::__field0, __variant) => {
                                    _serde::__private::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<A>(__variant),
                                        ArgDto::A,
                                    )
                                }
                                (__Field::__field1, __variant) => {
                                    _serde::__private::Result::map(
                                        _serde::de::VariantAccess::newtype_variant::<B>(__variant),
                                        ArgDto::B,
                                    )
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
                        __Visitor {
                            marker: _serde::__private::PhantomData::<ArgDto>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for ArgDto {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    ArgDto::A(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "A",
                            &__self_0,
                        )
                    }
                    ArgDto::B(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "B",
                            &__self_0,
                        )
                    }
                }
            }
        }
        impl nidrs::valid::validator::Validator for ArgDto {
            fn valid(&self) -> nidrs::valid::validator::ValidResult {
                use nidrs::valid::validator::Rule;
                use nidrs::valid::ruleset;
                use nidrs::valid::ruleset::*;
                match self {
                    ArgDto::A(v) => v.valid()?,
                    ArgDto::B(v) => v.valid()?,
                }
                return Ok(());
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
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    _serde::Serializer::serialize_newtype_struct(
                        __serializer,
                        "ArgWrapDto",
                        &self.0,
                    )
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
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
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
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "tuple struct ArgWrapDto",
                            )
                        }
                        #[inline]
                        fn visit_newtype_struct<__E>(
                            self,
                            __e: __E,
                        ) -> _serde::__private::Result<Self::Value, __E::Error>
                        where
                            __E: _serde::Deserializer<'de>,
                        {
                            let __field0: ArgDto = <ArgDto as _serde::Deserialize>::deserialize(
                                __e,
                            )?;
                            _serde::__private::Ok(ArgWrapDto(__field0))
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
                                ArgDto,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"tuple struct ArgWrapDto with 1 element",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(ArgWrapDto(__field0))
                        }
                    }
                    _serde::Deserializer::deserialize_newtype_struct(
                        __deserializer,
                        "ArgWrapDto",
                        __Visitor {
                            marker: _serde::__private::PhantomData::<ArgWrapDto>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for ArgWrapDto {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "ArgWrapDto",
                    &&self.0,
                )
            }
        }
        impl nidrs::valid::validator::Validator for ArgWrapDto {
            fn valid(&self) -> nidrs::valid::validator::ValidResult {
                use nidrs::valid::validator::Rule;
                use nidrs::valid::ruleset;
                use nidrs::valid::ruleset::*;
                return Ok(());
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
        use nidrs::macros::injectable;
        use nidrs::Inject;
        pub struct AppService {
            user_service: Inject<UserService>,
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
                ctx: nidrs::ModuleCtx,
                module_name: &str,
            ) -> nidrs::ModuleCtx {
                let service = ctx
                    .get_service::<UserService>(&module_name, "UserService");
                self.user_service.inject(service.clone());
                ctx
            }
        }
        impl nidrs::ImplMeta for AppService {
            fn __meta() -> nidrs::Meta {
                let mut meta = nidrs::Meta::new();
                meta.set_data(nidrs::datasets::ServiceType::Service);
                meta.set_data(nidrs::datasets::ServiceName("AppService"));
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
    use crate::modules::conf::ConfModule;
    use crate::modules::conf::ConfOptions;
    use crate::modules::log::LogModule;
    use crate::modules::user::UserModule;
    use controller::AppController;
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
            use nidrs::{
                Service, ControllerService, InterceptorService, InterCtx, Interceptor,
                ModuleCtx, StateCtx, ImplMeta,
            };
            if ctx.modules.contains_key("AppModule") {
                return ctx;
            }
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
            ctx.modules.insert("AppModule".to_string(), Box::new(self));
            ctx.imports
                .insert(
                    "AppModule".to_string(),
                    Vec::from([
                        "ConfModule".to_string(),
                        "LogModule".to_string(),
                        "UserModule".to_string(),
                    ]),
                );
            ctx.append_exports("AppModule", Vec::from(["AppService"]), false);
            ctx.register_interceptor(
                "AppModule",
                "LogInterceptor",
                Box::new(std::sync::Arc::new(crate::import::LogInterceptor::default())),
            );
            if ctx
                .register_controller(
                    "AppModule",
                    "AppController",
                    Box::new(std::sync::Arc::new(controller::AppController::default())),
                )
            {
                let t_controller = ctx
                    .get_controller::<
                        controller::AppController,
                    >("AppModule", "AppController");
                let t_interceptor_0 = ctx
                    .get_interceptor::<
                        crate::import::LogInterceptor,
                    >("AppModule", "LogInterceptor");
                let mut meta = nidrs::get_meta(t_controller.clone());
                let t_meta = t_controller.__meta_get_hello_world();
                meta.merge(t_meta);
                let version = *meta
                    .get::<&str>("version")
                    .unwrap_or(&ctx.defaults.default_version);
                let disable_default_prefix = meta
                    .get_data::<nidrs::datasets::DisableDefaultPrefix>()
                    .unwrap_or(&nidrs::datasets::DisableDefaultPrefix(false))
                    .value();
                let path = if disable_default_prefix {
                    "/hello".to_string()
                } else {
                    nidrs::template_format(
                        &{
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "{0}{1}",
                                    ctx.defaults.default_prefix,
                                    "/hello",
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
                meta.set_data(nidrs::datasets::RouterFullPath(path.clone()));
                let meta = std::sync::Arc::new(meta);
                let route_meta = meta.clone();
                let router = nidrs::externs::axum::Router::new()
                    .route(
                        &path,
                        nidrs::externs::axum::routing::get(|parts, p1| async move {
                            let mut t_meta = nidrs::Meta::new();
                            t_meta.extend(meta);
                            let t_body = nidrs_extern::axum::body::Body::empty();
                            let ctx = InterCtx {
                                meta: t_meta,
                                parts,
                                body: t_body,
                            };
                            let t_inter_fn_0 = |ctx: InterCtx<_>| async move {
                                let p0 = ctx.meta;
                                t_controller.get_hello_world(p0, p1).await
                            };
                            t_interceptor_0.interceptor(ctx, t_inter_fn_0).await
                        }),
                    );
                ctx.routers
                    .push(nidrs::RouterWrap {
                        router: router,
                        meta: route_meta.clone(),
                    });
                let t_controller = ctx
                    .get_controller::<
                        controller::AppController,
                    >("AppModule", "AppController");
                let t_interceptor_0 = ctx
                    .get_interceptor::<
                        crate::import::LogInterceptor,
                    >("AppModule", "LogInterceptor");
                let mut meta = nidrs::get_meta(t_controller.clone());
                let t_meta = t_controller.__meta_post_hello_world();
                meta.merge(t_meta);
                let version = *meta
                    .get::<&str>("version")
                    .unwrap_or(&ctx.defaults.default_version);
                let disable_default_prefix = meta
                    .get_data::<nidrs::datasets::DisableDefaultPrefix>()
                    .unwrap_or(&nidrs::datasets::DisableDefaultPrefix(false))
                    .value();
                let path = if disable_default_prefix {
                    "/hello".to_string()
                } else {
                    nidrs::template_format(
                        &{
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "{0}{1}",
                                    ctx.defaults.default_prefix,
                                    "/hello",
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
                meta.set_data(nidrs::datasets::RouterFullPath(path.clone()));
                let meta = std::sync::Arc::new(meta);
                let route_meta = meta.clone();
                let router = nidrs::externs::axum::Router::new()
                    .route(
                        &path,
                        nidrs::externs::axum::routing::post(|parts, p0, p1| async move {
                            let mut t_meta = nidrs::Meta::new();
                            t_meta.extend(meta);
                            let t_body = p1;
                            let ctx = InterCtx {
                                meta: t_meta,
                                parts,
                                body: t_body,
                            };
                            let t_inter_fn_0 = |ctx: InterCtx<_>| async move {
                                let t_body = ctx.body;
                                t_controller.post_hello_world(p0, t_body).await
                            };
                            t_interceptor_0.interceptor(ctx, t_inter_fn_0).await
                        }),
                    );
                ctx.routers
                    .push(nidrs::RouterWrap {
                        router: router,
                        meta: route_meta.clone(),
                    });
                let t_controller = ctx
                    .get_controller::<
                        controller::AppController,
                    >("AppModule", "AppController");
                let t_interceptor_0 = ctx
                    .get_interceptor::<
                        crate::import::LogInterceptor,
                    >("AppModule", "LogInterceptor");
                let mut meta = nidrs::get_meta(t_controller.clone());
                let t_meta = t_controller.__meta_get_hello_world2();
                meta.merge(t_meta);
                let version = *meta
                    .get::<&str>("version")
                    .unwrap_or(&ctx.defaults.default_version);
                let disable_default_prefix = meta
                    .get_data::<nidrs::datasets::DisableDefaultPrefix>()
                    .unwrap_or(&nidrs::datasets::DisableDefaultPrefix(false))
                    .value();
                let path = if disable_default_prefix {
                    "/hello2".to_string()
                } else {
                    nidrs::template_format(
                        &{
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "{0}{1}",
                                    ctx.defaults.default_prefix,
                                    "/hello2",
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
                meta.set_data(nidrs::datasets::RouterFullPath(path.clone()));
                let meta = std::sync::Arc::new(meta);
                let route_meta = meta.clone();
                let router = nidrs::externs::axum::Router::new()
                    .route(
                        &path,
                        nidrs::externs::axum::routing::get(|parts, p0| async move {
                            let mut t_meta = nidrs::Meta::new();
                            t_meta.extend(meta);
                            let t_body = nidrs_extern::axum::body::Body::empty();
                            let ctx = InterCtx {
                                meta: t_meta,
                                parts,
                                body: t_body,
                            };
                            let t_inter_fn_0 = |ctx: InterCtx<_>| async move {
                                t_controller.get_hello_world2(p0).await
                            };
                            t_interceptor_0.interceptor(ctx, t_inter_fn_0).await
                        }),
                    );
                ctx.routers
                    .push(nidrs::RouterWrap {
                        router: router,
                        meta: route_meta.clone(),
                    });
            }
            let svc = std::sync::Arc::new(AppService::default());
            ctx.register_service("AppModule", "AppService", Box::new(svc));
            let dyn_module = ConfModule::for_root(ConfOptions {
                log_level: "info".to_string(),
            });
            let mut dyn_module_services = dyn_module.services;
            dyn_module_services
                .drain()
                .for_each(|(k, v)| {
                    ctx.register_service("ConfModule", k, v);
                });
            let mut dyn_module_exports = dyn_module.exports;
            ctx.append_exports(
                "ConfModule",
                dyn_module_exports,
                nidrs::get_meta_by_type::<ConfModule>()
                    .get_data::<nidrs::datasets::Global>()
                    .unwrap_or(&nidrs::datasets::Global(false))
                    .value(),
            );
            let mut ctx = ConfModule::default().init(ctx);
            let mut ctx = LogModule::default().init(ctx);
            let mut ctx = UserModule::default().init(ctx);
            let t = ctx.get_service::<AppService>("AppModule", "AppService");
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
                    format_args!("Injecting {0}::{1}.\n", "AppModule", "AppService"),
                );
            };
            let ctx = t.inject(ctx, &"AppModule");
            let t = ctx.get_controller::<AppController>("AppModule", "AppController");
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
                    format_args!("Injecting {0}::{1}.\n", "AppModule", "AppController"),
                );
            };
            let ctx = t.inject(ctx, &"AppModule");
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
            meta.set_data(nidrs::datasets::ModuleName("AppModule"));
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
                    ctx: nidrs::ModuleCtx,
                    module_name: &str,
                ) -> nidrs::ModuleCtx {
                    ctx
                }
            }
            impl nidrs::ImplMeta for ConfOptions {
                fn __meta() -> nidrs::Meta {
                    let mut meta = nidrs::Meta::new();
                    meta.set_data(nidrs::datasets::ServiceType::Service);
                    meta.set_data(nidrs::datasets::ServiceName("ConfOptions"));
                    meta
                }
            }
        }
        pub mod service {
            use nidrs::macros::{injectable, on_module_init};
            use nidrs::{on_module_destroy, Inject};
            use super::options::ConfOptions;
            pub struct ConfService {
                pub options: Inject<ConfOptions>,
                pub log_level: String,
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
                    ctx: nidrs::ModuleCtx,
                    module_name: &str,
                ) -> nidrs::ModuleCtx {
                    let service = ctx
                        .get_service::<ConfOptions>(&module_name, "ConfOptions");
                    self.options.inject(service.clone());
                    ctx
                }
            }
            impl nidrs::ImplMeta for ConfService {
                fn __meta() -> nidrs::Meta {
                    let mut meta = nidrs::Meta::new();
                    meta.set_data(nidrs::datasets::ServiceType::Service);
                    meta.set_data(nidrs::datasets::ServiceName("ConfService"));
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
        use nidrs::{global, macros::module};
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
                use nidrs::{
                    Service, ControllerService, InterceptorService, InterCtx,
                    Interceptor, ModuleCtx, StateCtx, ImplMeta,
                };
                if ctx.modules.contains_key("ConfModule") {
                    return ctx;
                }
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
                ctx.modules.insert("ConfModule".to_string(), Box::new(self));
                ctx.imports.insert("ConfModule".to_string(), Vec::from([]));
                ctx.append_exports("ConfModule", Vec::from(["ConfService"]), true);
                ctx.register_interceptor(
                    "ConfModule",
                    "LogInterceptor",
                    Box::new(
                        std::sync::Arc::new(crate::import::LogInterceptor::default()),
                    ),
                );
                let svc = std::sync::Arc::new(ConfService::default());
                ctx.register_service("ConfModule", "ConfService", Box::new(svc));
                let t = ctx.get_service::<ConfService>("ConfModule", "ConfService");
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
                            "Injecting {0}::{1}.\n",
                            "ConfModule",
                            "ConfService",
                        ),
                    );
                };
                let ctx = t.inject(ctx, &"ConfModule");
                let service = ctx
                    .get_service::<ConfService>("ConfModule", "ConfService");
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
                            "Triggering event {0} for {1}::{2}.\n",
                            "on_module_init",
                            "ConfModule",
                            "ConfService",
                        ),
                    );
                };
                service.on_module_init();
                ctx
            }
            fn destroy(&self, ctx: &nidrs::ModuleCtx) {
                let service = ctx
                    .get_service::<ConfService>("ConfModule", "ConfService");
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
                            "Triggering event {0} for {1}::{2}.\n",
                            "on_module_destroy",
                            "ConfModule",
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
                meta.set_data(nidrs::datasets::Global(true));
                meta.set_data(nidrs::datasets::ModuleName("ConfModule"));
                meta
            }
        }
        impl ConfModule {
            pub fn for_root(options: ConfOptions) -> DynamicModule {
                DynamicModule::new().service(options)
            }
        }
    }
    pub mod log {
        use nidrs::{global, macros::module};
        pub mod interceptor {
            use std::{fmt::Debug, usize::MAX};
            use nidrs::externs::axum::{
                body::to_bytes, extract::FromRequest, http::Response,
                response::IntoResponse,
            };
            use nidrs::macros::interceptor;
            use nidrs::{
                AnyResponse, Inject, InterCtx, Interceptor, IntoAnyResponse, StateCtx,
            };
            use crate::{AppResult, CurrentUser};
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
            impl nidrs::InterceptorService for LogInterceptor {}
            impl nidrs::Service for LogInterceptor {
                fn inject(
                    &self,
                    ctx: nidrs::ModuleCtx,
                    module_name: &str,
                ) -> nidrs::ModuleCtx {
                    let service = ctx
                        .get_service::<LogService>(&module_name, "LogService");
                    self.log_service.inject(service.clone());
                    ctx
                }
            }
            impl nidrs::ImplMeta for LogInterceptor {
                fn __meta() -> nidrs::Meta {
                    let mut meta = nidrs::Meta::new();
                    meta.set_data(nidrs::datasets::ServiceType::Interceptor);
                    meta.set_data(nidrs::datasets::ServiceName("LogInterceptor"));
                    meta
                }
            }
            impl<B: FromRequest<StateCtx> + Debug, P: IntoResponse> Interceptor<B, P>
            for LogInterceptor {
                type R = AnyResponse;
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
                    {
                        ::std::io::_print(
                            format_args!(
                                "ctx: {0:?}\n",
                                ctx.parts.extensions.get::<CurrentUser>(),
                            ),
                        );
                    };
                    self.log_service.log("Before");
                    let r: AppResult<AnyResponse> = handler(ctx)
                        .await
                        .map(|r| AnyResponse::from_response(r));
                    if let Err(e) = &r {
                        return r;
                    }
                    let r = r.unwrap();
                    let (parts, body) = r.response.into_parts();
                    let body = to_bytes(body, MAX).await.unwrap();
                    {
                        ::std::io::_print(format_args!("body: {0:?}\n", body));
                    };
                    let body = {
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "{{\"data\":{0}}}",
                                String::from_utf8_lossy(&body),
                            ),
                        );
                        res
                    };
                    let response = Response::from_parts(parts, body.into());
                    self.log_service.log("After");
                    Ok(response.into())
                }
            }
        }
        pub mod service {
            use nidrs::macros::injectable;
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
                    ctx: nidrs::ModuleCtx,
                    module_name: &str,
                ) -> nidrs::ModuleCtx {
                    ctx
                }
            }
            impl nidrs::ImplMeta for LogService {
                fn __meta() -> nidrs::Meta {
                    let mut meta = nidrs::Meta::new();
                    meta.set_data(nidrs::datasets::ServiceType::Service);
                    meta.set_data(nidrs::datasets::ServiceName("LogService"));
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
        use interceptor::LogInterceptor;
        use service::LogService;
        pub struct LogModule;
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
                    Service, ControllerService, InterceptorService, InterCtx,
                    Interceptor, ModuleCtx, StateCtx, ImplMeta,
                };
                if ctx.modules.contains_key("LogModule") {
                    return ctx;
                }
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
                ctx.modules.insert("LogModule".to_string(), Box::new(self));
                ctx.imports.insert("LogModule".to_string(), Vec::from([]));
                ctx.append_exports("LogModule", Vec::from(["LogService"]), true);
                ctx.register_interceptor(
                    "LogModule",
                    "LogInterceptor",
                    Box::new(
                        std::sync::Arc::new(crate::import::LogInterceptor::default()),
                    ),
                );
                ctx.register_interceptor(
                    "LogModule",
                    "LogInterceptor",
                    Box::new(
                        std::sync::Arc::new(crate::import::LogInterceptor::default()),
                    ),
                );
                let svc = std::sync::Arc::new(LogService::default());
                ctx.register_service("LogModule", "LogService", Box::new(svc));
                let t = ctx.get_service::<LogService>("LogModule", "LogService");
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
                        format_args!("Injecting {0}::{1}.\n", "LogModule", "LogService"),
                    );
                };
                let ctx = t.inject(ctx, &"LogModule");
                let t = ctx
                    .get_interceptor::<LogInterceptor>("LogModule", "LogInterceptor");
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
                            "Injecting {0}::{1}.\n",
                            "LogModule",
                            "LogInterceptor",
                        ),
                    );
                };
                let ctx = t.inject(ctx, &"LogModule");
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
                        format_args!("Destroying module {0}.\n", "LogModule"),
                    );
                };
            }
        }
        impl nidrs::ImplMeta for LogModule {
            fn __meta() -> nidrs::Meta {
                let mut meta = nidrs::Meta::new();
                meta.set_data(nidrs::datasets::Global(true));
                meta.set_data(nidrs::datasets::ModuleName("LogModule"));
                meta
            }
        }
    }
    pub mod user {
        use nidrs::macros::module;
        pub mod controller {
            use std::collections::HashMap;
            use nidrs::macros::{controller, get};
            use nidrs::{
                externs::axum::extract::Query, post, valid::validator::Validator,
            };
            use nidrs::{AppResult, Inject};
            use nidrs_extern::axum::Json;
            use crate::modules::log::service::LogService;
            use super::{dto::CreateUserDto, service::UserService};
            pub struct UserController {
                user_service: Inject<UserService>,
                log_service: Inject<LogService>,
            }
            #[automatically_derived]
            impl ::core::default::Default for UserController {
                #[inline]
                fn default() -> UserController {
                    UserController {
                        user_service: ::core::default::Default::default(),
                        log_service: ::core::default::Default::default(),
                    }
                }
            }
            impl nidrs::ControllerService for UserController {}
            impl nidrs::Service for UserController {
                fn inject(
                    &self,
                    ctx: nidrs::ModuleCtx,
                    module_name: &str,
                ) -> nidrs::ModuleCtx {
                    let service = ctx
                        .get_service::<UserService>(&module_name, "UserService");
                    self.user_service.inject(service.clone());
                    let service = ctx
                        .get_service::<LogService>(&module_name, "LogService");
                    self.log_service.inject(service.clone());
                    ctx
                }
            }
            impl nidrs::ImplMeta for UserController {
                fn __meta() -> nidrs::Meta {
                    let mut meta = nidrs::Meta::new();
                    meta.set_data(nidrs::datasets::ServiceType::Controller);
                    meta.set_data(nidrs::datasets::ServiceName("UserController"));
                    meta.set_data(nidrs::datasets::ControllerPath("/user"));
                    meta
                }
            }
            impl UserController {
                pub async fn get_hello_world(
                    &self,
                    Query(q): Query<HashMap<String, String>>,
                ) -> AppResult<String> {
                    {
                        ::std::io::_print(format_args!("Query {0:?}\n", q));
                    };
                    self.log_service.log("hello");
                    Ok(self.user_service.extract().get_hello_world2())
                }
                pub fn __meta_get_hello_world(&self) -> nidrs::Meta {
                    let mut meta = nidrs::Meta::new();
                    meta.set_data(nidrs::datasets::RouterName("get_hello_world"));
                    meta.set_data(nidrs::datasets::RouterMethod("get"));
                    meta.set_data(nidrs::datasets::RouterPath("/hello"));
                    meta
                }
                pub async fn create_user(
                    &self,
                    dto: Json<CreateUserDto>,
                ) -> AppResult<String> {
                    dto.0.valid()?;
                    Ok(self.user_service.extract().get_hello_world2())
                }
                pub fn __meta_create_user(&self) -> nidrs::Meta {
                    let mut meta = nidrs::Meta::new();
                    meta.set_data(nidrs::datasets::RouterName("create_user"));
                    meta.set_data(nidrs::datasets::RouterMethod("post"));
                    meta.set_data(nidrs::datasets::RouterPath("/"));
                    meta
                }
            }
        }
        pub mod dto {
            use nidrs::valid_macro::dto;
            pub struct CreateUserDto {
                #[rule(Email, "age must be greater than 0")]
                pub name: String,
                #[rule(Number::default().max(12).min(0))]
                pub age: i32,
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for CreateUserDto {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "CreateUserDto",
                            false as usize + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "name",
                            &self.name,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "age",
                            &self.age,
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
                impl<'de> _serde::Deserialize<'de> for CreateUserDto {
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
                                    "name" => _serde::__private::Ok(__Field::__field0),
                                    "age" => _serde::__private::Ok(__Field::__field1),
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
                                    b"name" => _serde::__private::Ok(__Field::__field0),
                                    b"age" => _serde::__private::Ok(__Field::__field1),
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
                            marker: _serde::__private::PhantomData<CreateUserDto>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = CreateUserDto;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct CreateUserDto",
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
                                                &"struct CreateUserDto with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                    i32,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct CreateUserDto with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(CreateUserDto {
                                    name: __field0,
                                    age: __field1,
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
                                let mut __field1: _serde::__private::Option<i32> = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("age"),
                                                );
                                            }
                                            __field1 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
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
                                        _serde::__private::de::missing_field("name")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("age")?
                                    }
                                };
                                _serde::__private::Ok(CreateUserDto {
                                    name: __field0,
                                    age: __field1,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["name", "age"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "CreateUserDto",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<CreateUserDto>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[automatically_derived]
            impl ::core::fmt::Debug for CreateUserDto {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "CreateUserDto",
                        "name",
                        &self.name,
                        "age",
                        &&self.age,
                    )
                }
            }
            impl nidrs::valid::validator::Validator for CreateUserDto {
                fn valid(&self) -> nidrs::valid::validator::ValidResult {
                    use nidrs::valid::validator::Rule;
                    use nidrs::valid::ruleset;
                    use nidrs::valid::ruleset::*;
                    let v = &self.name;
                    Email
                        .valid(
                            v,
                            "name",
                            Some("age must be greater than 0".to_string()),
                        )?;
                    let v = &self.age;
                    Number::default().max(12).min(0).valid(v, "age", None)?;
                    return Ok(());
                }
            }
        }
        pub mod service {
            use std::sync::{Arc, Mutex};
            use nidrs::macros::injectable;
            use nidrs::Inject;
            use crate::app::service::AppService;
            pub struct UserService {
                app_service: Inject<AppService>,
                count: Arc<Mutex<i32>>,
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
                    ctx: nidrs::ModuleCtx,
                    module_name: &str,
                ) -> nidrs::ModuleCtx {
                    let service = ctx
                        .get_service::<AppService>(&module_name, "AppService");
                    self.app_service.inject(service.clone());
                    ctx
                }
            }
            impl nidrs::ImplMeta for UserService {
                fn __meta() -> nidrs::Meta {
                    let mut meta = nidrs::Meta::new();
                    meta.set_data(nidrs::datasets::ServiceType::Service);
                    meta.set_data(nidrs::datasets::ServiceName("UserService"));
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
        impl ::core::default::Default for UserModule {
            #[inline]
            fn default() -> UserModule {
                UserModule {}
            }
        }
        impl nidrs::Module for UserModule {
            fn init(self, mut ctx: nidrs::ModuleCtx) -> nidrs::ModuleCtx {
                use nidrs::{
                    Service, ControllerService, InterceptorService, InterCtx,
                    Interceptor, ModuleCtx, StateCtx, ImplMeta,
                };
                if ctx.modules.contains_key("UserModule") {
                    return ctx;
                }
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
                ctx.modules.insert("UserModule".to_string(), Box::new(self));
                ctx.imports
                    .insert(
                        "UserModule".to_string(),
                        Vec::from(["AppModule".to_string()]),
                    );
                ctx.append_exports("UserModule", Vec::from(["UserService"]), false);
                ctx.register_interceptor(
                    "UserModule",
                    "LogInterceptor",
                    Box::new(
                        std::sync::Arc::new(crate::import::LogInterceptor::default()),
                    ),
                );
                if ctx
                    .register_controller(
                        "UserModule",
                        "UserController",
                        Box::new(
                            std::sync::Arc::new(controller::UserController::default()),
                        ),
                    )
                {
                    let t_controller = ctx
                        .get_controller::<
                            controller::UserController,
                        >("UserModule", "UserController");
                    let t_interceptor_0 = ctx
                        .get_interceptor::<
                            crate::import::LogInterceptor,
                        >("UserModule", "LogInterceptor");
                    let mut meta = nidrs::get_meta(t_controller.clone());
                    let t_meta = t_controller.__meta_create_user();
                    meta.merge(t_meta);
                    let version = *meta
                        .get::<&str>("version")
                        .unwrap_or(&ctx.defaults.default_version);
                    let disable_default_prefix = meta
                        .get_data::<nidrs::datasets::DisableDefaultPrefix>()
                        .unwrap_or(&nidrs::datasets::DisableDefaultPrefix(false))
                        .value();
                    let path = if disable_default_prefix {
                        "/user/".to_string()
                    } else {
                        nidrs::template_format(
                            &{
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "{0}{1}",
                                        ctx.defaults.default_prefix,
                                        "/user/",
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
                    meta.set_data(nidrs::datasets::RouterFullPath(path.clone()));
                    let meta = std::sync::Arc::new(meta);
                    let route_meta = meta.clone();
                    let router = nidrs::externs::axum::Router::new()
                        .route(
                            &path,
                            nidrs::externs::axum::routing::post(|parts, p0| async move {
                                let mut t_meta = nidrs::Meta::new();
                                t_meta.extend(meta);
                                let t_body = p0;
                                let ctx = InterCtx {
                                    meta: t_meta,
                                    parts,
                                    body: t_body,
                                };
                                let t_inter_fn_0 = |ctx: InterCtx<_>| async move {
                                    let t_body = ctx.body;
                                    t_controller.create_user(t_body).await
                                };
                                t_interceptor_0.interceptor(ctx, t_inter_fn_0).await
                            }),
                        );
                    ctx.routers
                        .push(nidrs::RouterWrap {
                            router: router,
                            meta: route_meta.clone(),
                        });
                    let t_controller = ctx
                        .get_controller::<
                            controller::UserController,
                        >("UserModule", "UserController");
                    let t_interceptor_0 = ctx
                        .get_interceptor::<
                            crate::import::LogInterceptor,
                        >("UserModule", "LogInterceptor");
                    let mut meta = nidrs::get_meta(t_controller.clone());
                    let t_meta = t_controller.__meta_get_hello_world();
                    meta.merge(t_meta);
                    let version = *meta
                        .get::<&str>("version")
                        .unwrap_or(&ctx.defaults.default_version);
                    let disable_default_prefix = meta
                        .get_data::<nidrs::datasets::DisableDefaultPrefix>()
                        .unwrap_or(&nidrs::datasets::DisableDefaultPrefix(false))
                        .value();
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
                    meta.set_data(nidrs::datasets::RouterFullPath(path.clone()));
                    let meta = std::sync::Arc::new(meta);
                    let route_meta = meta.clone();
                    let router = nidrs::externs::axum::Router::new()
                        .route(
                            &path,
                            nidrs::externs::axum::routing::get(|parts, p0| async move {
                                let mut t_meta = nidrs::Meta::new();
                                t_meta.extend(meta);
                                let t_body = nidrs_extern::axum::body::Body::empty();
                                let ctx = InterCtx {
                                    meta: t_meta,
                                    parts,
                                    body: t_body,
                                };
                                let t_inter_fn_0 = |ctx: InterCtx<_>| async move {
                                    t_controller.get_hello_world(p0).await
                                };
                                t_interceptor_0.interceptor(ctx, t_inter_fn_0).await
                            }),
                        );
                    ctx.routers
                        .push(nidrs::RouterWrap {
                            router: router,
                            meta: route_meta.clone(),
                        });
                }
                let svc = std::sync::Arc::new(UserService::default());
                ctx.register_service("UserModule", "UserService", Box::new(svc));
                let mut ctx = AppModule::default().init(ctx);
                let t = ctx.get_service::<UserService>("UserModule", "UserService");
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
                            "Injecting {0}::{1}.\n",
                            "UserModule",
                            "UserService",
                        ),
                    );
                };
                let ctx = t.inject(ctx, &"UserModule");
                let t = ctx
                    .get_controller::<UserController>("UserModule", "UserController");
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
                            "Injecting {0}::{1}.\n",
                            "UserModule",
                            "UserController",
                        ),
                    );
                };
                let ctx = t.inject(ctx, &"UserModule");
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
                meta.set_data(nidrs::datasets::ModuleName("UserModule"));
                meta
            }
        }
    }
}
mod shared {
    pub mod fn_test {
        use nidrs::externs::anyhow;
        use nidrs::externs::axum::http::StatusCode;
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
                            "examples/hello/src/shared/fn_test.rs",
                            8usize,
                        ),
                    );
                    res
                },
            )?;
            Ok(())
        }
    }
}
use std::time::Duration;
use nidrs::externs::axum::{
    error_handling::HandleErrorLayer, extract::Request, http::StatusCode,
    middleware::{self, Next},
    response::Response, BoxError,
};
use nidrs::externs::tower::timeout::TimeoutLayer;
pub use nidrs::AppError;
pub use nidrs::AppResult;
fn main() {
    let app = nidrs::NidrsFactory::create(app::AppModule);
    let app = app.default_prefix("/api/{version}");
    let app = app.default_version("v1");
    let app = app
        .default_router_hook(|router_wrap| {
            {
                ::std::io::_print(
                    format_args!(
                        "router_wrap {0:?}\n",
                        (
                            router_wrap.meta.get_data::<nidrs::datasets::ServiceName>(),
                            router_wrap
                                .meta
                                .get_data::<nidrs::datasets::RouterFullPath>(),
                            router_wrap.meta.get::<&str>("router_name"),
                            router_wrap.meta.get::<&str>("controller_router_path"),
                            router_wrap.meta.get::<&str>("router_path"),
                        ),
                    ),
                );
            };
            if router_wrap.match_router_path("/**") {
                {
                    ::std::io::_print(format_args!("match /\n"));
                };
                router_wrap
                    .router
                    .layer(
                        nidrs::externs::tower::ServiceBuilder::new()
                            .layer(
                                HandleErrorLayer::new(|error: BoxError| async move {
                                    if error
                                        .is::<nidrs::externs::tower::timeout::error::Elapsed>()
                                    {
                                        Ok(StatusCode::REQUEST_TIMEOUT)
                                    } else {
                                        Err((
                                            StatusCode::INTERNAL_SERVER_ERROR,
                                            {
                                                let res = ::alloc::fmt::format(
                                                    format_args!("Unhandled internal error: {0}", error),
                                                );
                                                res
                                            },
                                        ))
                                    }
                                }),
                            )
                            .layer(TimeoutLayer::new(Duration::from_secs(5)))
                            .layer(middleware::from_fn(auth)),
                    )
            } else {
                router_wrap.router
            }
        });
    let app = app.listen(3000);
    app.block();
}
pub mod import {
    pub use crate::app::service::AppService;
    pub use crate::modules::log::interceptor::LogInterceptor;
    pub use crate::modules::log::service::LogService;
    pub use crate::modules::conf::service::ConfService;
    pub use crate::modules::conf::options::ConfOptions;
    pub use crate::modules::user::service::UserService;
    pub use crate::modules::user::controller::UserController;
    pub use crate::app::controller::AppController;
}
struct CurrentUser {
    pub id: u64,
    pub username: String,
}
#[automatically_derived]
impl ::core::clone::Clone for CurrentUser {
    #[inline]
    fn clone(&self) -> CurrentUser {
        CurrentUser {
            id: ::core::clone::Clone::clone(&self.id),
            username: ::core::clone::Clone::clone(&self.username),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for CurrentUser {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "CurrentUser",
            "id",
            &self.id,
            "username",
            &&self.username,
        )
    }
}
async fn auth(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    {
        ::std::io::_print(format_args!("auth {0:?}\n", req));
    };
    req.extensions_mut()
        .insert(CurrentUser {
            id: 1,
            username: "foo".to_string(),
        });
    Ok(next.run(req).await)
}
extern crate alloc;
