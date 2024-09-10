use nidrs::externs::axum::http::StatusCode;
use nidrs::{throw, Exception};

use crate::AppResult;

pub fn fn_test() -> AppResult {
    throw!(Exception::new(StatusCode::INTERNAL_SERVER_ERROR, nidrs::externs::anyhow::Error::msg("Error")));
    Ok(())
}
