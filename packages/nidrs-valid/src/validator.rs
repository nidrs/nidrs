use axum::Json;
use serde_json::Value;
use std::fmt;

pub type ValidResult = Result<(), ValidError>;

pub trait Rule<T> {
    fn valid(&self, value: T, field_path: &str, message: Option<String>) -> ValidResult;
    fn example(&self) -> Vec<Value>;
}

#[derive(thiserror::Error, Debug)]
pub struct ValidError {
    pub message: String,
}

impl fmt::Display for ValidError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub trait Validator {
    fn valid(&self) -> ValidResult;
    fn example(&self) -> Vec<Value>;
}

impl<T: Validator> Validator for Json<T> {
    fn valid(&self) -> ValidResult {
        self.0.valid()
    }

    fn example(&self) -> Vec<Value> {
        self.0.example()
    }
}

impl Validator for axum::body::Body {
    fn valid(&self) -> ValidResult {
        Ok(())
    }

    fn example(&self) -> Vec<Value> {
        vec![]
    }
}
