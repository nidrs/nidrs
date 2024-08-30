use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Float(pub f32);

impl TryFrom<&Value> for Float {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Float(v) => Ok(v.clone()),
            _ => Err(Error::new(proc_macro2::Span::call_site(), "Expected Expr")),
        }
    }
}

impl Deref for Float {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Float {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TryFrom<Transform<'_>> for def::Float {
    type Error = Error;

    fn try_from(value: Transform<'_>) -> Result<Self, Self::Error> {
        if let Value::Object(obj) = value.value {
            if let Some(Value::Float(v)) = obj.get(value.key) {
                return Ok(v.clone());
            }
        } else if let Value::Array(v) = value.value {
            let index = value.key.parse::<usize>().map_err(|_| Error::new(proc_macro2::Span::call_site(), "Expected usize"))?;
            if let Some(Value::Float(v)) = v.get(index) {
                return Ok(v.clone());
            }
        }

        Err(Error::new(proc_macro2::Span::call_site(), "TryFrom<Transform<'_>> for def::Float"))
    }
}
