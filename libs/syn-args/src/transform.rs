use crate::Value;

pub struct Transform<'a> {
    pub value: &'a Value,
    pub key: &'a str,
}

impl<'a> Transform<'a> {
    pub fn new(value: &'a Value, key: &'a str) -> Self {
        Self { value, key }
    }
}
