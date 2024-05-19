mod app;
mod modules;
mod shared;

pub use nidrs::AppError;
pub use nidrs::AppResult;

#[nidrs::main]
fn main() {
    let app = nidrs::NidrsFactory::create(app::AppModule);

    let app = app.default_prefix("/api/{version}");
    let app = app.default_version("v1");

    let app = app.listen(3000);

    // let mut sub_router = axum::Router::new();
    // for router in app.module_ctx.routers.iter() {
    //     sub_router = sub_router.merge(router.clone());
    // }
    // app.router = Router::new().nest("/t", sub_router);

    app.block();
}
