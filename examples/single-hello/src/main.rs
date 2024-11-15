mod app;
mod user;

use app::AppModule;

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
    let app = nidrs::NidrsFactory::create(AppModule);

    let app = app.default_prefix("/api/{version}");
    let app = app.default_version("v1");
    // let app = app.default_uses(app::interceptor::AppInterceptor);
    let app = app.default_layer(
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
    );

    let app = app.listen(3000);

    app.block();
}

#[derive(Clone, Debug)]
struct CurrentUser {
    pub id: u64,
    pub username: String,
}

async fn auth(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    println!("auth {:?}", req);

    req.extensions_mut().insert(CurrentUser { id: 1, username: "foo".to_string() });
    Ok(next.run(req).await)
}
