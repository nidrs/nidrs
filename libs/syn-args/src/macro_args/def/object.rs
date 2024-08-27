use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Object<T>(pub T);

impl<T> Deref for Object<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Object<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, T> TryInto<def::Object<T>> for Transform<'a>
where
    T: TryFrom<&'a Value, Error = Error>,
{
    type Error = Error;

    fn try_into(self) -> Result<def::Object<T>, Self::Error> {
        if let Value::Object(obj) = self.value {
            if let Some(v) = obj.get(self.key) {
                return Ok(def::Object(T::try_from(v)?));
            }
        }

        Err(Error::new(proc_macro2::Span::call_site(), "Expected Object"))
    }
}

impl TryFrom<&Value> for def::Object<HashMap<std::string::String, Value>> {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Object(obj) => {
                let mut res = HashMap::new();
                for (k, v) in obj.0.iter() {
                    res.insert(k.clone(), v.clone());
                }
                Ok(def::Object(res))
            }
            _ => Err(Error::new(proc_macro2::Span::call_site(), "Expected Object")),
        }
    }
}
