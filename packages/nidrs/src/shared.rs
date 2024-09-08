use crate::{AppError, AppResult};

pub fn otr<T>(option: Option<T>, error: &'static str) -> AppResult<T> {
    match option {
        Some(value) => Ok(value),
        None => Err(AppError::CompilationError(error.to_string())),
    }
}

/// Convert a path to OpenAPI format,eg: /hello/:id -> /hello/{id}
pub fn convert_path_to_openapi(path: &str) -> String {
    let mut result = String::new();
    let mut is_param = false;

    for c in path.chars() {
        if c == ':' {
            is_param = true;
            result.push('{');
        } else if is_param && c == '/' {
            is_param = false;
            result.push('}');
            result.push(c);
        } else {
            result.push(c);
        }
    }

    if is_param {
        result.push('}');
    }

    result
}

pub fn block<T>(t: T) -> T {
    t
}
