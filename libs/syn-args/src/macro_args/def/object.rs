use super::*;

#[derive(Debug, Clone)]
pub struct Object<T>(pub HashMap<std::string::String, T>);

impl<T: PartialEq> PartialEq for Object<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.keys().eq(other.0.keys()) && self.0.values().eq(other.0.values())
    }
}

impl<T> Deref for Object<T> {
    type Target = HashMap<std::string::String, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Object<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, T> TryFrom<Transform<'a>> for def::Object<T>
where
    T: TryFrom<&'a Value, Error = Error>,
{
    type Error = Error;

    fn try_from(value: Transform<'a>) -> Result<Self, Self::Error> {
        if let Value::Object(obj) = value.value {
            if let Some(v) = obj.get(value.key) {
                if let Value::Object(obj) = v {
                    let mut res = HashMap::new();
                    for (k, v) in obj.0.iter() {
                        res.insert(k.clone(), T::try_from(v)?);
                    }
                    return Ok(def::Object(res));
                }
            }
        }

        Err(Error::new(proc_macro2::Span::call_site(), "Expected TryFrom<Transform<'_>> for def::Object<T>"))
    }
}

impl TryFrom<&Value> for def::Object<Value> {
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
