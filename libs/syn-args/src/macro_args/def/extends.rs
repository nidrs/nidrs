use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Extends<T>(pub Vec<T>);

impl<T> Deref for Extends<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Extends<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TryFrom<&Value> for def::Extends<Value> {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        Err(Error::new(proc_macro2::Span::call_site(), "TryFrom<&Value> for def::Extends<Value>"))
    }
}

impl<'a, T> TryFrom<Transform<'a>> for def::Extends<T>
where
    T: TryFrom<&'a Value, Error = Error>,
{
    type Error = Error;

    fn try_from(value: Transform<'a>) -> Result<Self, Self::Error> {
        if let Value::Array(v) = value.value {
            let index = value.key.parse::<usize>().unwrap();
            let mut res = Vec::new();
            for i in index..v.len() {
                if let Some(v) = v.get(i) {
                    res.push(T::try_from(v)?);
                }
            }
            return Ok(Self(res));
        }
        // println!("Failed to parse: {:?}", (value.key, value.value));
        Err(Error::new(proc_macro2::Span::call_site(), "Expected Transform Into def::Extends"))
    }
}
