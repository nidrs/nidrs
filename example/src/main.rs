use axum::{http::StatusCode, response::IntoResponse, routing::get, Form, Router};
use nidrs::{Exception, StateCtx};
use nidrs_extern::colored::Colorize;


mod app;
mod conf;
mod user;
mod log;
mod shared;

pub use nidrs::AppResult;
pub use nidrs::AppError;

#[nidrs::main]
fn main() {
    let mut app = nidrs::NidrsFactory::create(app::AppModule);

    // app.router = Router::<StateCtx>::new().nest("/api", app.router);
    
    let app = app.listen(3000);
    let _ = tokio::runtime::Runtime::new().unwrap().block_on(app);
}


