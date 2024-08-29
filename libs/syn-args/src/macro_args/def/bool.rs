use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Bool(pub bool);

impl TryFrom<&Value> for Bool {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bool(v) => Ok(v.clone()),
            _ => Err(Error::new(proc_macro2::Span::call_site(), "Expected Expr")),
        }
    }
}

impl Deref for Bool {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Bool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> TryInto<def::Bool> for Transform<'a> {
    type Error = Error;

    #[cfg(feature = "loose_mode")]
    fn try_into(self) -> Result<def::Bool, Self::Error> {
        if let Value::Object(obj) = self.value {
            if let Some(Value::Bool(v)) = obj.get(self.key) {
                return Ok(v.clone());
            } else {
                return Ok(def::Bool(false));
            }
        }

        Err(Error::new(proc_macro2::Span::call_site(), "Expected Bool"))
    }

    #[cfg(not(feature = "loose_mode"))]
    fn try_into(self) -> Result<def::Bool, Self::Error> {
        if let Value::Object(obj) = self.value {
            if let Some(Value::Bool(v)) = obj.get(self.key) {
                return Ok(v.clone());
            }
        }

        Err(Error::new(proc_macro2::Span::call_site(), "Expected Bool"))
    }
}
