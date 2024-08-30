use super::*;

#[derive(Clone)]
pub struct Expr(pub syn::Expr);

impl<'a, T> From<T> for Expr
where
    T: Into<&'a str>,
{
    fn from(s: T) -> Self {
        Expr(syn::parse_str(s.into()).unwrap())
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Expr").field(&self.to_string()).finish()
    }
}

impl TryFrom<&Value> for Expr {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Expr(i) => Ok(i.clone()),
            _ => Err(Error::new(proc_macro2::Span::call_site(), "Expected TryFrom<&Value> for Expr")),
        }
    }
}

impl ToString for Expr {
    fn to_string(&self) -> std::string::String {
        self.0.to_token_stream().to_string()
    }
}

impl Deref for Expr {
    type Target = syn::Expr;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Expr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// impl<'a> TryInto<def::Expr> for Transform<'a> {
//     type Error = Error;

//     fn try_into(self) -> Result<def::Expr, Self::Error> {
//         if let Value::Object(obj) = self.value {
//             if let Some(Value::Expr(v)) = obj.get(self.key) {
//                 return Ok(v.clone());
//             }
//         }

//         Err(Error::new(proc_macro2::Span::call_site(), "Expected TryInto<def::Expr> for Transform"))
//     }
// }

impl TryFrom<Transform<'_>> for def::Expr {
    type Error = Error;

    fn try_from(value: Transform<'_>) -> Result<Self, Self::Error> {
        if let Value::Object(obj) = value.value {
            if let Some(Value::Expr(v)) = obj.get(value.key) {
                return Ok(v.clone());
            }
        } else if let Value::Array(v) = value.value {
            let index = value.key.parse::<usize>().map_err(|_| Error::new(proc_macro2::Span::call_site(), "Expected usize"))?;
            if let Some(Value::Expr(v)) = v.get(index) {
                return Ok(v.clone());
            }
        }

        Err(Error::new(proc_macro2::Span::call_site(), "Expected TryFrom<Transform> for def::Expr"))
    }
}

impl Expr {
    pub fn to_parts_path(&self) -> Result<syn::Path, syn::Error> {
        match &self.0 {
            syn::Expr::Path(path) => Ok(path.path.clone()),
            syn::Expr::Call(call) => {
                if let syn::Expr::Path(path) = &*call.func {
                    Ok(path.path.clone())
                } else {
                    Err(Error::new(proc_macro2::Span::call_site(), "Expected Path"))
                }
            }
            _ => Err(Error::new(proc_macro2::Span::call_site(), "Expected Path")),
        }
    }
}
