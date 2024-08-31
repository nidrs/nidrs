use crate::Value;

/// This is an intermediate type used to extract the key attribute from the value. The value must be either a Hash or a Vec.
pub struct Transform<'a> {
    pub value: &'a Value,
    pub key: &'a str,
}

impl<'a> Transform<'a> {
    pub fn new(value: &'a Value, key: &'a str) -> Self {
        Self { value, key }
    }
}
