use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Int(pub i32);

impl TryFrom<&Value> for Int {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int(v) => Ok(v.clone()),
            _ => Err(Error::new(proc_macro2::Span::call_site(), "Expected PathIdent")),
        }
    }
}

impl Deref for Int {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Int {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> TryInto<def::Int> for Transform<'a> {
    type Error = Error;

    fn try_into(self) -> Result<def::Int, Self::Error> {
        if let Value::Object(obj) = self.value {
            if let Some(Value::Int(v)) = obj.get(self.key) {
                return Ok(v.clone());
            }
        }

        Err(Error::new(proc_macro2::Span::call_site(), "Expected Int"))
    }
}
