use std::ops::{Deref, DerefMut};

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Null;

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

impl TryFrom<&Value> for Ident {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Ident(i) => Ok(i.clone()),
            _ => Err(Error::new(proc_macro2::Span::call_site(), "Expected Ident")),
        }
    }
}

impl Deref for Ident {
    type Target = std::string::String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Ident {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Int(pub i32);

impl TryFrom<&Value> for Int {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int(v) => Ok(v.clone()),
            _ => Err(Error::new(proc_macro2::Span::call_site(), "Expected Ident")),
        }
    }
}

impl Deref for Int {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Int {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Float(pub f32);

impl TryFrom<&Value> for Float {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Float(v) => Ok(v.clone()),
            _ => Err(Error::new(proc_macro2::Span::call_site(), "Expected Ident")),
        }
    }
}

impl Deref for Float {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Float {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Bool(pub bool);

impl TryFrom<&Value> for Bool {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bool(v) => Ok(v.clone()),
            _ => Err(Error::new(proc_macro2::Span::call_site(), "Expected Ident")),
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

#[derive(Debug, PartialEq, Clone)]
pub struct String(pub std::string::String);

impl TryFrom<&Value> for String {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(v) => Ok(v.clone()),
            _ => Err(Error::new(proc_macro2::Span::call_site(), "Expected Ident")),
        }
    }
}

impl Deref for String {
    type Target = std::string::String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for String {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

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
