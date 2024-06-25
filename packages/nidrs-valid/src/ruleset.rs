use serde_json::{json, Value};

use crate::validator::{Rule, ValidError, ValidResult, Validator};

#[derive(Debug, Default)]
pub struct Email;

impl Rule<&str> for Email {
    fn valid(&self, value: &str, field_path: &str, message: Option<String>) -> ValidResult {
        if value.contains('@') && value.contains('.') {
            Ok(())
        } else {
            Err(ValidError { message: message.unwrap_or_else(|| format!("{} is not email", field_path)) })
        }
    }
    fn example(&self) -> Vec<Value> {
        vec![json!("1111@qq.com")]
    }
}

#[derive(Debug, Default)]
pub struct Number {
    max_limit: Option<isize>,
    min_limit: Option<isize>,
}

impl Number {
    pub fn max(mut self, v: isize) -> Self {
        self.max_limit = Some(v);
        self
    }

    pub fn min(mut self, v: isize) -> Self {
        self.min_limit = Some(v);
        self
    }
}

impl Rule<&isize> for Number {
    fn valid(&self, value: &isize, field_path: &str, message: Option<String>) -> ValidResult {
        let field_path = field_path.to_string();
        if let Some(max) = self.max_limit {
            if *value > max {
                return Err(ValidError { message: message.unwrap_or_else(|| format!("{} > min", field_path)) });
            }
        }
        if let Some(min) = self.min_limit {
            if *value < min {
                return Err(ValidError { message: message.unwrap_or_else(|| format!("{} > max", field_path)) });
            }
        }
        Ok(())
    }
    fn example(&self) -> Vec<Value> {
        vec![json!(122)]
    }
}

impl Rule<&i32> for Number {
    fn valid(&self, value: &i32, field_path: &str, message: Option<String>) -> ValidResult {
        let value = *value as isize;
        Number::valid(self, &value, field_path, message)
    }

    fn example(&self) -> Vec<Value> {
        vec![json!(122)]
    }
}


pub struct Valid<'a, T: Validator>(pub &'a T);

impl<'a, T: Validator> Rule<&T> for Valid<'a, T> {
    fn valid(&self, value: &T, field_path: &str, message: Option<String>) -> ValidResult {
        self.0.valid()
    }
    fn example(&self) -> Vec<Value> {
        vec![]
    }
}