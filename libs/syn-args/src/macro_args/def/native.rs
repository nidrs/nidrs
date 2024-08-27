// impl<T> TryInto<std::option::Option<T>> for Transform<'_> {
//     type Error = Error;

//     fn try_into(self) -> Result<std::option::Option<T>, Self::Error> {
//         if let Value::Object(obj) = self.value {
//             if let Some(v) = obj.get(self.key) {
//                 return Ok(Some(v.clone()));
//             } else {
//                 return Ok(None);
//             }
//         }
//         Err(Error::new(proc_macro2::Span::call_site(), "Expected std::option::Option"))
//     }
// }

// impl TryFrom<&Value> for std::option::Option<Value> {
//     type Error = Error;

//     fn try_from(value: &Value) -> Result<Self, Self::Error> {
//         match value {
//             Value::Option(opt) => Ok(Some(opt.0.as_ref().map(|v| v.as_ref().clone()).unwrap_or(Value::Null))),
//             _ => Err(Error::new(proc_macro2::Span::call_site(), "Expected std::option::Option")),
//         }
//     }
// }
