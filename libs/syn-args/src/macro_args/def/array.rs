use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Array<Item>(pub Vec<Item>);

impl<T> Array<T> {
    pub fn merge(mut self, b: Array<T>) -> Self {
        self.0.extend(b.0);
        self
    }
}

impl<Item> Deref for Array<Item> {
    type Target = Vec<Item>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Item> DerefMut for Array<Item> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, T> TryFrom<Transform<'a>> for def::Array<T>
where
    T: TryFrom<&'a Value, Error = Error> + Debug,
{
    type Error = Error;

    #[cfg(not(feature = "loose_mode"))]
    fn try_from(value: Transform<'a>) -> Result<Self, Self::Error> {
        if let Value::Object(obj) = value.value {
            if let Some(Value::Array(v)) = obj.get(value.key) {
                let mut res = vec![];
                for v in v.0.iter() {
                    let item = T::try_from(v).unwrap();
                    res.push(item);
                }
                return Ok(Self(res));
            }
        } else if let Value::Array(v) = value.value {
            let index = value.key.parse::<usize>().map_err(|_| Error::new(proc_macro2::Span::call_site(), "Expected usize"))?;
            if let Some(Value::Array(v)) = v.get(index) {
                let mut res = vec![];
                // println!("Array: {:?}", v);
                for v in v.0.iter() {
                    let item = T::try_from(v)?;
                    // println!("Array: end");
                    res.push(item);
                }
                return Ok(Self(res));
            }
        }
        // println!("Failed to parse: {:?}", (value.key, value.value));
        Err(Error::new(proc_macro2::Span::call_site(), "Expected Transform Into def::Array"))
    }

    #[cfg(feature = "loose_mode")]
    fn try_from(value: Transform<'a>) -> Result<Self, Self::Error> {
        if let Value::Object(obj) = value.value {
            if let Some(Value::Array(v)) = obj.get(value.key) {
                let mut res = vec![];
                for v in v.0.iter() {
                    let item = T::try_from(v).unwrap();
                    res.push(item);
                }
                return Ok(Self(res));
            } else {
                return Ok(Self(vec![]));
            }
        } else if let Value::Array(v) = value.value {
            let index = value.key.parse::<usize>().unwrap();
            if let Some(Value::Array(v)) = v.get(index) {
                let mut res = vec![];
                // println!("Array: {:?}", v);
                for v in v.0.iter() {
                    let item = T::try_from(v).unwrap();
                    // println!("Array: end");
                    res.push(item);
                }
                return Ok(Self(res));
            }
        }

        Err(Error::new(proc_macro2::Span::call_site(), "Expected Array"))
    }
}

// impl<'a, T> TryInto<def::Array<T>> for Transform<'a>
// where
//     T: TryFrom<&'a Value, Error = Error>,
// {
//     type Error = Error;

//     #[cfg(feature = "loose_mode")]
//     fn try_into(self) -> Result<def::Array<T>, Self::Error> {
//         if let Value::Object(obj) = self.value {
//             if let Some(Value::Array(arr)) = obj.get(self.key) {
//                 return Ok(def::Array(arr.iter().map(|v| T::try_from(v)).collect::<Result<Vec<T>, Self::Error>>()?));
//             } else {
//                 return Ok(def::Array(Vec::new()));
//             }
//         } else if let Value::Array(v) = self.value {
//             println!("Array: {:?}", v);
//             let index = self.key.parse::<usize>().unwrap();
//             if let Some(value) = v.get(index) {
//                 return Ok(def::Array(vec![T::try_from(value)?]));
//             }
//         }

//         Err(Error::new(proc_macro2::Span::call_site(), "Expected Array"))
//     }

//     #[cfg(not(feature = "loose_mode"))]
//     fn try_into(self) -> Result<def::Array<T>, Self::Error> {
//         if let Value::Object(obj) = self.value {
//             if let Some(Value::Array(arr)) = obj.get(self.key) {
//                 return Ok(def::Array(arr.iter().map(|v| T::try_from(v)).collect::<Result<Vec<T>, Self::Error>>()?));
//             }
//         }

//         Err(Error::new(proc_macro2::Span::call_site(), "Expected Array"))
//     }
// }

impl<'a, Item> TryFrom<&'a Value> for def::Array<Item>
where
    Item: TryFrom<&'a Value, Error = syn::Error>,
{
    type Error = syn::Error;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::Array(arr) => {
                let mut res = vec![];
                for v in arr.0.iter() {
                    let item: Item = Item::try_from(v)?;
                    res.push(item);
                }
                Ok(def::Array(res))
            }
            _ => Err(Error::new(proc_macro2::Span::call_site(), "Expected Array")),
        }
    }
}
