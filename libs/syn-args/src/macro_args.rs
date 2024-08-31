use std::{
    collections::HashMap,
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use quote::ToTokens;
use syn::Error;

use crate::SynArgs;

/// Contains basic types for syn-args, used for quick parameter parsing
pub mod def;

/// Contains some auxiliary functions
pub mod utils;

/// parse the input into Arguments
pub struct Formal {}

impl Default for Formal {
    fn default() -> Self {
        Self::new()
    }
}

impl Formal {
    pub fn new() -> Self {
        Formal {}
    }

    pub fn parse(&self, input: &str) -> Result<Arguments, Error> {
        let expr = syn::parse_str::<SynArgs>(input).unwrap();
        // println!("Formal: {:#?}", expr);
        Ok(Arguments(expr.value))
    }
}

/// Intermediate layer type, usually converted from syn::Expr
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Expr(def::Expr),
    Int(def::Int),
    Float(def::Float),
    Bool(def::Bool),
    String(def::String),
    Option(def::Option<Box<Value>>),
    Object(def::Object<Value>),
    Array(def::Array<Value>),
}

/// Arguments type, usually converted from Value
/// This type is used to distinguish top-level types for easier processing, and is ultimately converted to specific types through Arguments.
#[derive(Debug, Clone, PartialEq)]
pub struct Arguments(pub Value);

impl Deref for Arguments {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Arguments {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TryFrom<Value> for Arguments {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        Ok(Arguments(value))
    }
}
