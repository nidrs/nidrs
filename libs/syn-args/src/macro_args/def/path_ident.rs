use syn::Path;

use super::*;

#[derive(Clone)]
pub struct PathIdent(pub Path);

impl<'a, T> From<T> for PathIdent
where
    T: Into<&'a str>,
{
    fn from(s: T) -> Self {
        PathIdent(syn::parse_str(s.into()).unwrap())
    }
}

impl PartialEq for PathIdent {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Debug for PathIdent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("PathIdent").field(&self.to_string()).finish()
    }
}

impl TryFrom<&Value> for PathIdent {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::PathIdent(i) => Ok(i.clone()),
            _ => Err(Error::new(proc_macro2::Span::call_site(), "Expected PathIdent")),
        }
    }
}

impl ToString for PathIdent {
    fn to_string(&self) -> std::string::String {
        self.0.to_token_stream().to_string()
    }
}

impl Deref for PathIdent {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PathIdent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> TryInto<def::PathIdent> for Transform<'a> {
    type Error = Error;

    fn try_into(self) -> Result<def::PathIdent, Self::Error> {
        if let Value::Object(obj) = self.value {
            if let Some(Value::PathIdent(v)) = obj.get(self.key) {
                return Ok(v.clone());
            }
        }

        Err(Error::new(proc_macro2::Span::call_site(), "Expected PathIdent"))
    }
}
