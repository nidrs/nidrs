use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Bool(pub bool);

impl TryFrom<&Value> for Bool {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bool(v) => Ok(v.clone()),
            _ => Err(Error::new(proc_macro2::Span::call_site(), "Expected Bool")),
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

// impl<'a> TryInto<def::Bool> for Transform<'a> {
//     type Error = Error;

//     #[cfg(feature = "loose_mode")]
//     fn try_into(self) -> Result<def::Bool, Self::Error> {
//         if let Value::Object(obj) = self.value {
//             if let Some(Value::Bool(v)) = obj.get(self.key) {
//                 return Ok(v.clone());
//             } else {
//                 return Ok(def::Bool(false));
//             }
//         }

//         Err(Error::new(proc_macro2::Span::call_site(), "Expected Bool"))
//     }

//     #[cfg(not(feature = "loose_mode"))]
//     fn try_into(self) -> Result<def::Bool, Self::Error> {
//         if let Value::Object(obj) = self.value {
//             if let Some(Value::Bool(v)) = obj.get(self.key) {
//                 return Ok(v.clone());
//             }
//         }

//         Err(Error::new(proc_macro2::Span::call_site(), "Expected Bool"))
//     }
// }

impl TryFrom<Transform<'_>> for def::Bool {
    type Error = Error;

    #[cfg(not(feature = "loose_mode"))]
    fn try_from(value: Transform<'_>) -> Result<Self, Self::Error> {
        if let Value::Object(obj) = value.value {
            if let Some(Value::Bool(v)) = obj.get(value.key) {
                return Ok(v.clone());
            }
        } else if let Value::Array(v) = value.value {
            let index = value.key.parse::<usize>().map_err(|_| Error::new(proc_macro2::Span::call_site(), "Expected usize"))?;
            if let Some(Value::Bool(v)) = v.get(index) {
                return Ok(v.clone());
            }
        }

        Err(Error::new(proc_macro2::Span::call_site(), "Expected TryFrom<Transform> for def::Bool"))
    }

    #[cfg(feature = "loose_mode")]
    fn try_from(value: Transform<'_>) -> Result<Self, Self::Error> {
        if let Value::Object(obj) = value.value {
            if let Some(Value::Bool(v)) = obj.get(value.key) {
                return Ok(v.clone());
            } else {
                return Ok(def::Bool(false));
            }
        }

        Err(Error::new(proc_macro2::Span::call_site(), "Expected TryFrom<Transform> for def::Bool"))
    }
}
