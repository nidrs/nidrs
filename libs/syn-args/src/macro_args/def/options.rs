use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Option<T>(pub std::option::Option<Box<T>>);

impl<T> Deref for Option<T> {
    type Target = std::option::Option<Box<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Option<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, T> TryFrom<Transform<'a>> for def::Option<T>
where
    T: TryFrom<&'a Value, Error = Error>,
{
    type Error = Error;

    fn try_from(value: Transform<'a>) -> Result<Self, Self::Error> {
        if let Value::Object(obj) = value.value {
            if let Some(v) = obj.get(value.key) {
                return Ok(Self(Some(Box::new(T::try_from(v)?))));
            } else {
                return Ok(Self(None));
            }
        }
        Err(Error::new(proc_macro2::Span::call_site(), "Expected Option"))
    }
}

impl TryFrom<&Value> for def::Option<Value> {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Option(opt) => Ok(opt.clone()),
            _ => Err(Error::new(proc_macro2::Span::call_site(), "Expected Option")),
        }
    }
}
