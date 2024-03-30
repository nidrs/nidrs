use axum::{routing::get, Router};
use nidrs::StateCtx;

mod app;
mod conf;
mod user;

fn main() {
    let mut app = nidrs::NestFactory::create(app::AppModule);

    app.router = app.router.merge(Router::<StateCtx>::new().route("/api", get(|| async { "Hello, World!" })));
    
    let app = app.listen::<AppError>(3000);
    let _ = tokio::runtime::Runtime::new().unwrap().block_on(app);
}



#[derive(Clone, Debug, Default)]
pub struct AppState{}

pub enum AppError {
    
}


impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        // Convert std::io::Error to AppError here
        // Example: AppError::new(error.to_string())
        unimplemented!()
    }
}