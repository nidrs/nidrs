use super::*;

#[derive(Debug, PartialEq)]
pub struct Null;
impl IntoArgType for Null {
    fn into_arg_type() -> Type {
        Type::Null
    }
}

#[derive(Debug, PartialEq)]
pub struct Ident(pub std::string::String);
impl IntoArgType for Ident {
    fn into_arg_type() -> Type {
        Type::Ident
    }
}

#[derive(Debug, PartialEq)]
pub struct Int(pub i32);
impl IntoArgType for Int {
    fn into_arg_type() -> Type {
        Type::Int
    }
}

#[derive(Debug, PartialEq)]
pub struct Float;
impl IntoArgType for Float {
    fn into_arg_type() -> Type {
        Type::Float
    }
}

#[derive(Debug, PartialEq)]
pub struct Bool;
impl IntoArgType for Bool {
    fn into_arg_type() -> Type {
        Type::Bool
    }
}

#[derive(Debug, PartialEq)]
pub struct String;
impl IntoArgType for String {
    fn into_arg_type() -> Type {
        Type::String
    }
}

#[derive(Debug, PartialEq)]
pub struct Object<Item> {
    pub props: Item,
}
impl<Item> IntoArgType for Object<Item> {
    fn into_arg_type() -> Type {
        Type::Object(HashMap::new())
    }
}

#[derive(Debug, PartialEq)]
pub struct Array<Item> {
    pub items: Item,
}
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
