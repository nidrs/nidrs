use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Int(pub i32);

impl TryFrom<&Value> for Int {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int(v) => Ok(v.clone()),
            _ => Err(Error::new(proc_macro2::Span::call_site(), "Expected Expr")),
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

impl TryFrom<Transform<'_>> for def::Int {
    type Error = Error;

    fn try_from(value: Transform<'_>) -> Result<Self, Self::Error> {
        if let Value::Object(obj) = value.value {
            if let Some(Value::Int(v)) = obj.get(value.key) {
                return Ok(v.clone());
            }
        } else if let Value::Array(v) = value.value {
            let index = value.key.parse::<usize>().map_err(|_| Error::new(proc_macro2::Span::call_site(), "Expected usize"))?;
            if let Some(Value::Int(v)) = v.get(index) {
                return Ok(v.clone());
            }
        }

        Err(Error::new(proc_macro2::Span::call_site(), "Expected TryFrom<Transform> for def::Int"))
    }
}
