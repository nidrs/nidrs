mod app;
mod modules;
mod shared;

use std::time::Duration;

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

#[nidrs::main]
fn main() {
    let app = nidrs::NidrsFactory::create(app::AppModule);

    let app = app.default_prefix("/api/{version}");
    let app = app.default_version("v1");

    let app = app.default_router_hook(|router_wrap| {
        println!(
            "router_wrap {:?}",
            (
                router_wrap.meta.get_data::<nidrs::metadata::ServiceName>(),
                router_wrap.meta.get_data::<nidrs::metadata::RouterFullPath>(),
                router_wrap.meta.get::<&str>("router_name"),
                router_wrap.meta.get::<&str>("controller_router_path"),
                router_wrap.meta.get::<&str>("router_path")
            )
        );

        if router_wrap.match_router_path("/**") {
            println!("match /");
            router_wrap.router.layer(
                nidrs::externs::tower::ServiceBuilder::new()
                    .layer(HandleErrorLayer::new(|error: BoxError| async move {
                        if error.is::<nidrs::externs::tower::timeout::error::Elapsed>() {
                            Ok(StatusCode::REQUEST_TIMEOUT)
                        } else {
                            Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Unhandled internal error: {error}")))
                        }
                    }))
                    .layer(TimeoutLayer::new(Duration::from_secs(5)))
                    .layer(middleware::from_fn(auth)),
            )
        } else {
            router_wrap.router
        }
    });

    let app = app.listen(3000);

    // app.router = app.router.route_layer(middleware::from_fn(auth));
    // app.router = app.router.layer(ServiceBuilder::new().layer(middleware::from_fn(auth)));
    // app.router = app.router.layer(
    //     nidrs::externs::tower::ServiceBuilder::new()
    //         .layer(HandleErrorLayer::new(|error: BoxError| async move {
    //             if error.is::<nidrs::externs::tower::timeout::error::Elapsed>() {
    //                 Ok(StatusCode::REQUEST_TIMEOUT)
    //             } else {
    //                 Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Unhandled internal error: {error}")))
    //             }
    //         }))
    //         .layer(TimeoutLayer::new(Duration::from_secs(5)))
    //         .layer(middleware::from_fn(auth))
    // );

    // let mut sub_router = axum::Router::new();
    // for router in app.module_ctx.routers.iter() {
    //     sub_router = sub_router.merge(router.clone());
    // }
    // app.router = Router::new().nest("/t", sub_router);

    app.block();
}

#[derive(Clone, Debug)]
struct CurrentUser {
    pub id: u64,
    pub username: String,
}

async fn auth(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    // let auth_header = req.headers().get(http::header::AUTHORIZATION).and_then(|header| header.to_str().ok());

    // let auth_header = if let Some(auth_header) = auth_header {
    //     auth_header
    // } else {
    //     return Err(StatusCode::UNAUTHORIZED);
    // };

    println!("auth {:?}", req);

    req.extensions_mut().insert(CurrentUser { id: 1, username: "foo".to_string() });
    Ok(next.run(req).await)
}
