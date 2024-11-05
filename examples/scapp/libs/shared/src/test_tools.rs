use nidrs::externs::anyhow;
use nidrs::externs::axum::http::StatusCode;
use nidrs::{throw, Exception};

use nidrs::AppResult;

pub fn fn_test() -> AppResult {
    throw!(Exception::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        anyhow::Error::msg("Error")
    ));
    Ok(())
}
