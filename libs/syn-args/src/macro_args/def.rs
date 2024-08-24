use std::ops::{Deref, DerefMut};

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Null;
// impl IntoArgType for Null {
//     fn into_arg_type() -> Type {
//         Type::Null
//     }
// }
impl TryFrom<&Value> for Null {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(Null),
            _ => Err(Error::new(proc_macro2::Span::call_site(), "Expected Ident")),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ident(pub std::string::String);
// impl IntoArgType for Ident {
//     fn into_arg_type() -> Type {
//         Type::Ident
//     }
// }
impl TryFrom<&Value> for Ident {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Ident(i) => Ok(i.clone()),
            _ => Err(Error::new(proc_macro2::Span::call_site(), "Expected Ident")),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Int(pub i32);
// impl IntoArgType for Int {
//     fn into_arg_type() -> Type {
//         Type::Int
//     }
// }
impl TryFrom<&Value> for Int {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int(v) => Ok(v.clone()),
            _ => Err(Error::new(proc_macro2::Span::call_site(), "Expected Ident")),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Float(pub f32);
// impl IntoArgType for Float {
//     fn into_arg_type() -> Type {
//         Type::Float
//     }
// }
impl TryFrom<&Value> for Float {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Float(v) => Ok(v.clone()),
            _ => Err(Error::new(proc_macro2::Span::call_site(), "Expected Ident")),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Bool(pub bool);
// impl IntoArgType for Bool {
//     fn into_arg_type() -> Type {
//         Type::Bool
//     }
// }
impl TryFrom<&Value> for Bool {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bool(v) => Ok(v.clone()),
            _ => Err(Error::new(proc_macro2::Span::call_site(), "Expected Ident")),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct String(pub std::string::String);
// impl IntoArgType for String {
//     fn into_arg_type() -> Type {
//         Type::String
//     }
// }
impl TryFrom<&Value> for String {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(v) => Ok(v.clone()),
            _ => Err(Error::new(proc_macro2::Span::call_site(), "Expected Ident")),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Object<T>(pub T);
// impl<Item> IntoArgType for Object<Item> {
//     fn into_arg_type() -> Type {
//         Type::Object(HashMap::new())
//     }
// }

#[derive(Debug, PartialEq, Clone)]
pub struct Array<Item>(pub Vec<Item>);

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

// impl<Item> IntoArgType for Array<Item> {
//     fn into_arg_type() -> Type {
//         Type::Array(vec![])
//     }
// }

// pub trait IntoArgType {
//     fn into_arg_type() -> Type;
// }

// #[derive(Debug)]
// pub enum Type {
//     Null,
//     Ident,
//     Int,
//     Float,
//     Bool,
//     String,
//     Object(HashMap<std::string::String, Value>),
//     Array(Vec<Value>),
// }
