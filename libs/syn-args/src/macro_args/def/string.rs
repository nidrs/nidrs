use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct String(pub std::string::String);

impl TryFrom<&Value> for String {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(v) => Ok(v.clone()),
            _ => Err(Error::new(proc_macro2::Span::call_site(), "Expected PathIdent")),
        }
    }
}

impl Deref for String {
    type Target = std::string::String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for String {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> TryInto<def::String> for Transform<'a> {
    type Error = Error;

    fn try_into(self) -> Result<def::String, Self::Error> {
        if let Value::Object(obj) = self.value {
            if let Some(Value::String(v)) = obj.get(self.key) {
                return Ok(v.clone());
            }
        }

        Err(Error::new(proc_macro2::Span::call_site(), "Expected String"))
    }
}
