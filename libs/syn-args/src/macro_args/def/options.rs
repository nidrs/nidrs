use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Option<T>(pub std::option::Option<T>);

impl<T> Deref for Option<T> {
    type Target = std::option::Option<T>;

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
                return Ok(Self(Some(T::try_from(v)?)));
            } else {
                return Ok(Self(None));
            }
        }
        if let Value::Array(v) = value.value {
            let index = value.key.parse::<usize>().map_err(|_| Error::new(proc_macro2::Span::call_site(), "Expected usize"))?;
            if let Some(v) = v.get(index) {
                return Ok(Self(Some(T::try_from(v)?)));
            } else {
                return Ok(Self(None));
            }
        }
        // println!("Failed to parse: {:?}", (value.key, value.value));
        Err(Error::new(proc_macro2::Span::call_site(), "Expected Transform Into def::Option"))
    }
}

impl TryFrom<&Value> for def::Option<Box<Value>> {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Option(opt) => Ok(opt.clone()),
            _ => Err(Error::new(proc_macro2::Span::call_site(), "Expected &Value Into def::Option<Box<Value>>")),
        }
    }
}
