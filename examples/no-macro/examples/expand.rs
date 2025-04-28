#![feature(print_internals)]
#![feature(panic_internals)]
#![feature(alloc)]
#![feature(fmt_helpers_for_derive)]
#![allow(warnings, unused)]
#![feature(hint_must_use)]
#![feature(liballoc_internals)]
// >>Push: Global("app") -- [None]
//  CMETA: []
// >>Push: Service("AppController") -- [None]
//  CMETA: ["ServiceType"]
//  CMETA: ["ServiceName"]
//  CMETA: ["ControllerPath"]
// service_derive "AppController"
// >>Push: Handler("get") -- [None]
//  CMETA: ["RouterName"]
//  CMETA: ["RouterMethod"]
//  CMETA: ["RouterPath"]
// route_derive "get"
// route_derive is_tuple false
// << Pop: Some(Handler("get")) ["RouterName", "handler", "RouterMethod", "RouterPath", "ServiceType", "service", "ServiceName", "ControllerPath", "global"]

// << Pop: Some(Service("AppController")) ["ServiceType", "service", "ServiceName", "ControllerPath", "global"]

// >>Push: Service("AppModule") -- [None]
//  CMETA: ["__"]
// module "AppModule"
// << Pop: Some(Service("AppModule")) ["__", "service", "global"]
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod app {
    use nidrs::{controller, get, module, AppResult, Module, ModuleCtx};
    pub struct AppController {}
    #[automatically_derived]
    impl ::core::default::Default for AppController {
        #[inline]
        fn default() -> AppController {
            AppController {}
        }
    }
    impl nidrs::Controller for AppController {}
    impl nidrs::Service for AppController {
        fn inject(&self, ctx: nidrs::ModuleCtx, module_name: &str) -> nidrs::ModuleCtx {
            ctx
        }
    }
    impl nidrs::ImplMeta for AppController {
        fn __meta(&self) -> nidrs::InnerMeta {
            let mut meta = nidrs::InnerMeta::new();
            meta.set_data(nidrs::datasets::ServiceType::from("Controller"));
            meta.set("service", "AppController");
            meta.set_data(nidrs::datasets::ServiceName::from("AppController"));
            meta.set_data(nidrs::datasets::ControllerPath::from("/app"));
            meta.set("global", "app");
            meta
        }
    }
    impl AppController {
        pub async fn get(&self) -> AppResult<String> {
            Ok("hello".to_string())
        }
        pub fn __meta_get(&self) -> nidrs::InnerMeta {
            let mut meta = nidrs::InnerMeta::new();
            meta.set_data(nidrs::datasets::RouterName::from("get"));
            meta.set("handler", "get");
            meta.set_data(nidrs::datasets::RouterMethod::from("get"));
            meta.set_data(nidrs::datasets::RouterPath::from("/hello"));
            meta.set_data(nidrs::datasets::ServiceType::from("Controller"));
            meta.set("service", "AppController");
            meta.set_data(nidrs::datasets::ServiceName::from("AppController"));
            meta.set_data(nidrs::datasets::ControllerPath::from("/app"));
            meta.set("global", "app");
            meta
        }
        pub fn __route_get(&self, mut ctx: nidrs::ModuleCtx, module: &str) -> nidrs::ModuleCtx {
            use axum::response::IntoResponse;
            use nidrs::externs::axum;
            use nidrs::externs::axum::{extract::Query, Json};
            use nidrs::externs::meta::{InnerMeta, Meta};
            use nidrs::Interceptor;
            use serde_json::Value;
            let mut meta = self.__meta_get();
            let router_info = ctx.get_router_full(&meta);
            if let Err(e) = router_info {
                {
                    ::core::panicking::panic_fmt(format_args!("[{0}] {1:?}", "__route_get", e));
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
            let module_name = module;
            let controller_name = meta.get_data::<nidrs::datasets::ServiceName>().unwrap().value();
            let t_controller = ctx.get_controller::<Self>(module_name, controller_name);
            let router = nidrs::externs::axum::Router::new()
                .route(
                    &full_path,
                    nidrs::externs::axum::routing::get(|| async move {
                        let r = t_controller.get().await;
                        match r {
                            Ok(r) => Json(r).into_response(),
                            Err(e) => e.into_response(),
                        }
                    }),
                )
                .route_layer(nidrs::externs::axum::Extension(meta.clone()));
            ctx.routers.push(nidrs::MetaRouter::new(router, meta));
            ctx
        }
    }
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
            ctx.imports.insert("AppModule".to_string(), Vec::from([]));
            ctx.append_exports("AppModule", Vec::<&str>::from([]), false);
            if ctx.register_controller("AppModule", "AppController", Box::new(std::sync::Arc::new(AppController::default()))) {
                let t_controller = ctx.get_controller::<AppController>("AppModule", "AppController");
                ctx = t_controller.__route_get(ctx, "AppModule");
            }
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
            meta.set("__", true);
            meta.set("service", "AppModule");
            meta.set("global", "app");
            meta
        }
    }
}
use app::AppModule;
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
    let app = nidrs::NidrsFactory::create(AppModule);
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
    pub use crate::app::AppController;
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
