use super::*;

#[derive(Debug, PartialEq)]
pub struct Null;
impl IntoArgType for Null {
    fn into_arg_type() -> Type {
        Type::Null
    }
}
impl TryFrom<&Value> for Null {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(Null),
            _ => Err(Error::new(proc_macro2::Span::call_site(), "Expected Ident")),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Ident(pub std::string::String);
impl IntoArgType for Ident {
    fn into_arg_type() -> Type {
        Type::Ident
    }
}
impl TryFrom<&Value> for Ident {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Ident(i) => Ok(Ident(i.clone())),
            _ => Err(Error::new(proc_macro2::Span::call_site(), "Expected Ident")),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Int(pub i32);
impl IntoArgType for Int {
    fn into_arg_type() -> Type {
        Type::Int
    }
}
impl TryFrom<&Value> for Int {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int(v) => Ok(Int(*v)),
            _ => Err(Error::new(proc_macro2::Span::call_site(), "Expected Ident")),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Float(pub f32);
impl IntoArgType for Float {
    fn into_arg_type() -> Type {
        Type::Float
    }
}
impl TryFrom<&Value> for Float {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Float(f32) => Ok(Float(*f32)),
            _ => Err(Error::new(proc_macro2::Span::call_site(), "Expected Ident")),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Bool(pub bool);
impl IntoArgType for Bool {
    fn into_arg_type() -> Type {
        Type::Bool
    }
}
impl TryFrom<&Value> for Bool {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bool(b) => Ok(Bool(*b)),
            _ => Err(Error::new(proc_macro2::Span::call_site(), "Expected Ident")),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct String(pub std::string::String);
impl IntoArgType for String {
    fn into_arg_type() -> Type {
        Type::String
    }
}
impl TryFrom<&Value> for String {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(s) => Ok(String(s.clone())),
            _ => Err(Error::new(proc_macro2::Span::call_site(), "Expected Ident")),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Object<Item>(pub HashMap<std::string::String, Item>);
impl<Item> IntoArgType for Object<Item> {
    fn into_arg_type() -> Type {
        Type::Object(HashMap::new())
    }
}

#[derive(Debug, PartialEq)]
pub struct Array<Item>(pub Vec<Item>);
impl<Item> IntoArgType for Array<Item> {
    fn into_arg_type() -> Type {
        Type::Array(vec![])
    }
}

pub trait IntoArgType {
    fn into_arg_type() -> Type;
}

#[derive(Debug)]
pub enum Type {
    Null,
    Ident,
    Int,
    Float,
    Bool,
    String,
    Object(HashMap<std::string::String, Value>),
    Array(Vec<Value>),
}
