use axum::{extract::path, http::StatusCode, response::IntoResponse, routing::get, Form, Router};
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
    let app = nidrs::NidrsFactory::create(app::AppModule);

    let app = app.default_prefix("/api/{version}");
    let app = app.default_version("v1");

    app.listen(3000);
}
