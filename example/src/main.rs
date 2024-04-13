use axum::{http::StatusCode, response::IntoResponse, routing::get, Form, Router};
use nidrs::{Exception, StateCtx};
use nidrs_extern::colored::Colorize;


mod app;
mod conf;
mod user;
mod log;
mod shared;


#[nidrs::main]
fn main() {
    let mut app = nidrs::NidrsFactory::create(app::AppModule);

    app.router = app.router.merge(Router::<StateCtx>::new().route("/api", get(|| async { "Hello, World!" })));
    
    let app = app.listen::<AppError>(3000);
    let _ = tokio::runtime::Runtime::new().unwrap().block_on(app);
}



#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Environment variable not found")]
    EnvironmentVariableNotFound(#[from] std::env::VarError),
    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    Exception(#[from] Exception),
}
impl IntoResponse for AppError{
    fn into_response(self) -> axum::response::Response {
        axum::response::Json(serde_json::json!({
            "code": 500,
            "message": self.to_string(),
        }))
        .into_response()
    }
}

pub type AppResult<T = ()> = Result<T, AppError>;
