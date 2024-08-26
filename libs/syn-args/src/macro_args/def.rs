use std::ops::{Deref, DerefMut};

use syn::Path;

use crate::Transform;

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Null;

impl TryFrom<&Value> for Null {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Null => Ok(Null),
            _ => Err(Error::new(proc_macro2::Span::call_site(), "Expected PathIdent")),
        }
    }
}

impl<'a> TryInto<def::Null> for Transform<'a> {
    type Error = Error;

    fn try_into(self) -> Result<def::Null, Self::Error> {
        if let Value::Object(obj) = self.value {
            if let Some(Value::Null) = obj.get(self.key) {
                return Ok(def::Null);
            }
        }

        Err(Error::new(proc_macro2::Span::call_site(), "Expected Null"))
    }
}

#[derive(Clone)]
pub struct PathIdent(pub Path);

impl<'a, T> From<T> for PathIdent
where
    T: Into<&'a str>,
{
    fn from(s: T) -> Self {
        PathIdent(syn::parse_str(s.into()).unwrap())
    }
}

impl PartialEq for PathIdent {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Debug for PathIdent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("PathIdent").field(&self.to_string()).finish()
    }
}

impl TryFrom<&Value> for PathIdent {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::PathIdent(i) => Ok(i.clone()),
            _ => Err(Error::new(proc_macro2::Span::call_site(), "Expected PathIdent")),
        }
    }
}

impl ToString for PathIdent {
    fn to_string(&self) -> std::string::String {
        self.0.to_token_stream().to_string()
    }
}

impl Deref for PathIdent {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PathIdent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> TryInto<def::PathIdent> for Transform<'a> {
    type Error = Error;

    fn try_into(self) -> Result<def::PathIdent, Self::Error> {
        if let Value::Object(obj) = self.value {
            if let Some(Value::PathIdent(v)) = obj.get(self.key) {
                return Ok(v.clone());
            }
        }

        Err(Error::new(proc_macro2::Span::call_site(), "Expected PathIdent"))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Int(pub i32);

impl TryFrom<&Value> for Int {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int(v) => Ok(v.clone()),
            _ => Err(Error::new(proc_macro2::Span::call_site(), "Expected PathIdent")),
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

impl<'a> TryInto<def::Int> for Transform<'a> {
    type Error = Error;

    fn try_into(self) -> Result<def::Int, Self::Error> {
        if let Value::Object(obj) = self.value {
            if let Some(Value::Int(v)) = obj.get(self.key) {
                return Ok(v.clone());
            }
        }

        Err(Error::new(proc_macro2::Span::call_site(), "Expected Int"))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Float(pub f32);

impl TryFrom<&Value> for Float {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Float(v) => Ok(v.clone()),
            _ => Err(Error::new(proc_macro2::Span::call_site(), "Expected PathIdent")),
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

impl<'a> TryInto<def::Float> for Transform<'a> {
    type Error = Error;

    fn try_into(self) -> Result<def::Float, Self::Error> {
        if let Value::Object(obj) = self.value {
            if let Some(Value::Float(v)) = obj.get(self.key) {
                return Ok(v.clone());
            }
        }

        Err(Error::new(proc_macro2::Span::call_site(), "Expected Float"))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Bool(pub bool);

impl TryFrom<&Value> for Bool {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bool(v) => Ok(v.clone()),
            _ => Err(Error::new(proc_macro2::Span::call_site(), "Expected PathIdent")),
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

impl<'a> TryInto<def::Bool> for Transform<'a> {
    type Error = Error;

    fn try_into(self) -> Result<def::Bool, Self::Error> {
        if let Value::Object(obj) = self.value {
            if let Some(Value::Bool(v)) = obj.get(self.key) {
                return Ok(v.clone());
            }
        }

        Err(Error::new(proc_macro2::Span::call_site(), "Expected Bool"))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct String(pub std::string::String);

impl TryFrom<&Value> for String {
    type Error = Error;

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(v) => Ok(v.clone()),
            _ => Err(Error::new(proc_macro2::Span::call_site(), "Expected PathIdent")),
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

impl<'a> TryInto<def::String> for Transform<'a> {
    type Error = Error;

    fn try_into(self) -> Result<def::String, Self::Error> {
        if let Value::Object(obj) = self.value {
            if let Some(Value::String(v)) = obj.get(self.key) {
                return Ok(v.clone());
            }
        }

        Err(Error::new(proc_macro2::Span::call_site(), "Expected String"))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Option<T>(pub std::option::Option<Box<T>>);

impl<T> Deref for Option<T> {
    type Target = std::option::Option<Box<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Option<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a, T> TryFrom<Transform<'a>> for def::Option<T>
where
    T: TryFrom<&'a Value, Error = Error>,
{
    type Error = Error;

    fn try_from(value: Transform<'a>) -> Result<Self, Self::Error> {
        if let Value::Object(obj) = value.value {
            if let Some(v) = obj.get(value.key) {
                return Ok(Self(Some(Box::new(T::try_from(v)?))));
            } else {
                return Ok(Self(None));
            }
        }
        Err(Error::new(proc_macro2::Span::call_site(), "Expected Array"))
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

impl<'a, T> TryInto<def::Array<T>> for Transform<'a>
where
    T: TryFrom<&'a Value, Error = Error>,
{
    type Error = Error;

    fn try_into(self) -> Result<def::Array<T>, Self::Error> {
        if let Value::Object(obj) = self.value {
            if let Some(Value::Array(arr)) = obj.get(self.key) {
                return Ok(def::Array(arr.iter().map(|v| T::try_from(v)).collect::<Result<Vec<T>, Self::Error>>()?));
            }
        }

        Err(Error::new(proc_macro2::Span::call_site(), "Expected Array"))
    }
}
