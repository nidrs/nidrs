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
    // app.default_version(1);
    
    // app.router = Router::<StateCtx>::new().nest("/api", app.router);

    let a1 = 123;
    
    app.listen(3000);
}



fn f(path: &str) {
    let ctx: Ctx = Ctx{ a1: 12 };
    println!("{}",format!("{a1} {a2}", a1 = ctx.a1, a2 = ctx.a1));
}

struct Ctx{
    a1: i32
}