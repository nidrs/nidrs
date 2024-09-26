use axum::Json;
use nidrs_extern::axum::{http::StatusCode, response::IntoResponse};
use nidrs_extern::{colored::Colorize, *};

#[cfg(feature = "valid")]
use nidrs_valid::validator::ValidError;

pub type AppResult<T = ()> = Result<T, AppError>;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Environment variable not found")]
    EnvironmentVariableNotFound(#[from] std::env::VarError),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),

    #[error(transparent)]
    TokioJoinError(#[from] tokio::task::JoinError),

    #[cfg(feature = "valid")]
    #[error(transparent)]
    ValidError(#[from] ValidError),

    #[error("Compilation Error")]
    CompilationError(String),

    #[error(transparent)]
    Exception(#[from] Exception),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let t = format!("Error: {:?}", self);
        nidrs_macro::elog!("{}", t);
        if let AppError::Exception(e) = self {
            return e.into_response();
        }
        axum::response::Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR).body(t).unwrap().into_response()
    }
}

pub fn __throw<E: Into<AppError>>(e: E, line: &str) -> AppResult<()> {
    let e = e.into();
    if let AppError::Exception(mut e) = e {
        e.line = line.to_string();
        print!("{}", "[nidrs] Exception ".red().bold());
        println!("{}", e.to_string().red().bold());
        return Err(e.into());
    }
    Err(e)
}

#[derive(thiserror::Error, Debug)]
pub struct Exception {
    pub status: StatusCode,
    pub error: anyhow::Error,
    pub line: String,
}

impl Exception {
    pub fn new(status: StatusCode, error: anyhow::Error) -> Self {
        Exception { status, error, line: String::new() }
    }
}

impl std::fmt::Display for Exception {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HTTP Exception: status={}, error={}\n   {}", self.status, self.error, self.line)
    }
}

impl IntoResponse for Exception {
    fn into_response(self) -> axum::response::Response {
        axum::response::Response::builder().status(self.status).body(self.error.to_string()).unwrap().into_response()
    }
}

pub fn result_to_json<T: IntoResponse>(result: AppResult<T>) -> Json<axum::response::Response> {
    match result {
        Ok(t) => Json(t.into_response()),
        Err(e) => Json(e.into_response()),
    }
}
