use crate::Transform;

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Null;

impl TryFrom<&Value> for Null {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(Null),
            _ => Err(Error::new(proc_macro2::Span::call_site(), "Expected PathIdent")),
        }
    }
}

impl<'a> TryInto<def::Null> for Transform<'a> {
    type Error = Error;

    fn try_into(self) -> Result<def::Null, Self::Error> {
        if let Value::Object(obj) = self.value {
            if let Some(Value::Null) = obj.get(self.key) {
                return Ok(def::Null);
            }
        }

        Err(Error::new(proc_macro2::Span::call_site(), "Expected Null"))
    }
}
