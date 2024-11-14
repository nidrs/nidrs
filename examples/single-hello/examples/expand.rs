#![feature(print_internals)]
#![feature(panic_internals)]
#![feature(alloc)]
#![feature(fmt_helpers_for_derive)]
#![allow(warnings, unused)]
#![feature(hint_must_use)]
#![feature(liballoc_internals)]
// >>Push: Global("app") -- [None]
//  CMETA: []
// >>Push: Service("AppModule") -- [None]
//  CMETA: ["__"]
// << Pop: Some(Service("AppModule")) ["__", "service", "global"]

#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod app {
    use nidrs::{controller, get, module, AppResult, Module, ModuleCtx};
    impl AppController {}
}
use app::AppModule;
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
    let app = nidrs::NidrsFactory::create(AppModule);
    let app = app.default_prefix("/api/{version}");
    let app = app.default_version("v1");
    let app = app
        .default_layer(
            nidrs::externs::tower::ServiceBuilder::new()
                .layer(
                    HandleErrorLayer::new(|error: BoxError| async move {
                        if error.is::<nidrs::externs::tower::timeout::error::Elapsed>() {
                            Ok(StatusCode::REQUEST_TIMEOUT)
                        } else {
                            Err((
                                StatusCode::INTERNAL_SERVER_ERROR,
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Unhandled internal error: {0}", error),
                                    );
                                    res
                                }),
                            ))
                        }
                    }),
                )
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
