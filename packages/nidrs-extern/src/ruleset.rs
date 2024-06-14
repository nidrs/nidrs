use serde_json::Value;

use crate::validator::{Rule, ValidError, ValidResult};

#[derive(Debug, Default)]
pub struct Email;

impl Rule<&str> for Email {
    fn valid(&self, value: &str, message: Option<&str>) -> ValidResult {
        if value.contains("@") && value.contains(".") {
            Ok(())
        } else {
            Err(ValidError { message: message.unwrap_or_else(|| "email format error").to_string() })
        }
    }
    fn example(&self) -> Vec<Value> {
        return vec![Value::String("1111@qq.com".into())];
    }
}

#[derive(Debug, Default)]
pub struct Number;

impl Rule<i32> for Number {
    fn valid(&self, value: i32, message: Option<&str>) -> ValidResult {
        if value > 0 {
            Ok(())
        } else {
            Err(ValidError { message: message.unwrap_or_else(|| "email format error").to_string() })
        }
    }
    fn example(&self) -> Vec<Value> {
        return vec![Value::String("1111@qq.com".into())];
    }
}

impl Rule<&String> for Number {
    fn valid(&self, value: &String, message: Option<&str>) -> ValidResult {
        if value.contains("@") && value.contains(".") {
            Ok(())
        } else {
            Err(ValidError { message: message.unwrap_or_else(|| "email format error").to_string() })
        }
    }
    fn example(&self) -> Vec<Value> {
        return vec![Value::String("1111@qq.com".into())];
    }
}
