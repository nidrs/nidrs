use serde_json::Value;

use crate::validator::{Rule, ValidError, ValidResult};

#[derive(Debug, Default)]
pub struct Email {
    pub message: Option<String>,
}

impl Rule<&str> for Email {
    fn set_message(mut self, message: &str) -> Self {
        self.message = Some(message.into());
        self
    }
    fn valid(self, value: &str) -> ValidResult {
        if value.contains("@") && value.contains(".com") {
            Ok(())
        } else {
            Err(ValidError { message: self.message.unwrap_or_else(|| "email format error".into()) })
        }
    }
    fn example(&self) -> Vec<Value> {
        return vec![Value::String("1111@qq.com".into())];
    }
}
