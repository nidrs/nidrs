use crate::{AppError, AppResult};

pub fn otr<T>(option: Option<T>, error: &'static str) -> AppResult<T> {
    match option {
        Some(value) => Ok(value),
        None => Err(AppError::CompilationError(error.to_string())),
    }
}
