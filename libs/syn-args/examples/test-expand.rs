#![feature(print_internals)]
#![feature(structural_match)]
#![feature(core_intrinsics)]
#![feature(panic_internals)]
#![feature(rustc_attrs)]
#![feature(alloc)]
#![feature(fmt_helpers_for_derive)]
#![allow(warnings, unused)]
#![feature(hint_must_use)]
#![feature(liballoc_internals)]
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use syn_args::{def, derive::ArgsParse, ArgsParse, Formal, Value};
pub enum ModuleArgs {
    F1(def::Int, def::Int),
    F2(def::Int),
    F3(def::Expr),
    F4(def::Array<def::Expr>),
    F5(ModuleSubObj),
    F6(def::Array<ModuleSubObj>),
    F7(SubWrap),
    F8(def::Option<T1>),
}
#[automatically_derived]
impl ::core::fmt::Debug for ModuleArgs {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match self {
            ModuleArgs::F1(__self_0, __self_1) => ::core::fmt::Formatter::debug_tuple_field2_finish(f, "F1", __self_0, &__self_1),
            ModuleArgs::F2(__self_0) => ::core::fmt::Formatter::debug_tuple_field1_finish(f, "F2", &__self_0),
            ModuleArgs::F3(__self_0) => ::core::fmt::Formatter::debug_tuple_field1_finish(f, "F3", &__self_0),
            ModuleArgs::F4(__self_0) => ::core::fmt::Formatter::debug_tuple_field1_finish(f, "F4", &__self_0),
            ModuleArgs::F5(__self_0) => ::core::fmt::Formatter::debug_tuple_field1_finish(f, "F5", &__self_0),
            ModuleArgs::F6(__self_0) => ::core::fmt::Formatter::debug_tuple_field1_finish(f, "F6", &__self_0),
            ModuleArgs::F7(__self_0) => ::core::fmt::Formatter::debug_tuple_field1_finish(f, "F7", &__self_0),
            ModuleArgs::F8(__self_0) => ::core::fmt::Formatter::debug_tuple_field1_finish(f, "F8", &__self_0),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for ModuleArgs {}
#[automatically_derived]
impl ::core::cmp::PartialEq for ModuleArgs {
    #[inline]
    fn eq(&self, other: &ModuleArgs) -> bool {
        let __self_discr = ::core::intrinsics::discriminant_value(self);
        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
        __self_discr == __arg1_discr
            && match (self, other) {
                (ModuleArgs::F1(__self_0, __self_1), ModuleArgs::F1(__arg1_0, __arg1_1)) => __self_0 == __arg1_0 && __self_1 == __arg1_1,
                (ModuleArgs::F2(__self_0), ModuleArgs::F2(__arg1_0)) => __self_0 == __arg1_0,
                (ModuleArgs::F3(__self_0), ModuleArgs::F3(__arg1_0)) => __self_0 == __arg1_0,
                (ModuleArgs::F4(__self_0), ModuleArgs::F4(__arg1_0)) => __self_0 == __arg1_0,
                (ModuleArgs::F5(__self_0), ModuleArgs::F5(__arg1_0)) => __self_0 == __arg1_0,
                (ModuleArgs::F6(__self_0), ModuleArgs::F6(__arg1_0)) => __self_0 == __arg1_0,
                (ModuleArgs::F7(__self_0), ModuleArgs::F7(__arg1_0)) => __self_0 == __arg1_0,
                (ModuleArgs::F8(__self_0), ModuleArgs::F8(__arg1_0)) => __self_0 == __arg1_0,
                _ => unsafe { ::core::intrinsics::unreachable() },
            }
    }
}
impl TryFrom<&syn_args::Value> for ModuleArgs {
    type Error = syn::Error;
    fn try_from(v: &syn_args::Value) -> Result<Self, Self::Error> {
        if let syn_args::Value::Array(_) = v {
            if let Ok(rt) = syn_args::utils::ewc::<_, _, syn::Error>(|| {
                Ok(ModuleArgs::F1(syn_args::Transform::new(v, "0").try_into()?, syn_args::Transform::new(v, "1").try_into()?))
            }) {
                return Ok(rt);
            }
            if let Ok(rt) = syn_args::utils::ewc::<_, _, syn::Error>(|| Ok(ModuleArgs::F2(syn_args::Transform::new(v, "0").try_into()?))) {
                return Ok(rt);
            }
            if let Ok(rt) = syn_args::utils::ewc::<_, _, syn::Error>(|| Ok(ModuleArgs::F3(syn_args::Transform::new(v, "0").try_into()?))) {
                return Ok(rt);
            }
            if let Ok(rt) = syn_args::utils::ewc::<_, _, syn::Error>(|| Ok(ModuleArgs::F4(syn_args::Transform::new(v, "0").try_into()?))) {
                return Ok(rt);
            }
            if let Ok(rt) = syn_args::utils::ewc::<_, _, syn::Error>(|| Ok(ModuleArgs::F5(syn_args::Transform::new(v, "0").try_into()?))) {
                return Ok(rt);
            }
            if let Ok(rt) = syn_args::utils::ewc::<_, _, syn::Error>(|| Ok(ModuleArgs::F6(syn_args::Transform::new(v, "0").try_into()?))) {
                return Ok(rt);
            }
            if let Ok(rt) = syn_args::utils::ewc::<_, _, syn::Error>(|| Ok(ModuleArgs::F7(syn_args::Transform::new(v, "0").try_into()?))) {
                return Ok(rt);
            }
            if let Ok(rt) = syn_args::utils::ewc::<_, _, syn::Error>(|| Ok(ModuleArgs::F8(syn_args::Transform::new(v, "0").try_into()?))) {
                return Ok(rt);
            }
        }
        Err(Self::Error::new(
            proc_macro2::Span::call_site(),
            ::alloc::__export::must_use({
                let res = ::alloc::fmt::format(format_args!("Invalid args try_from {0}", "ModuleArgs"));
                res
            }),
        ))
    }
}
impl TryFrom<syn_args::Value> for ModuleArgs {
    type Error = syn::Error;
    fn try_from(v: syn_args::Value) -> Result<Self, Self::Error> {
        ModuleArgs::try_from(&v)
    }
}
impl syn_args::ArgsParse for ModuleArgs {
    fn parse(input: &str) -> Result<Self, syn::Error> {
        syn_args::Formal::new().parse(input)?.try_into()
    }
}
impl TryFrom<syn_args::Transform<'_>> for ModuleArgs {
    type Error = syn::Error;
    fn try_from(value: syn_args::Transform) -> Result<Self, Self::Error> {
        if let syn_args::Value::Object(obj) = value.value {
            if let Some(v) = obj.get(value.key) {
                return v.try_into();
            }
        } else if let syn_args::Value::Array(v) = value.value {
            let index = value.key.parse::<usize>().unwrap();
            if let Some(value) = v.get(index) {
                return Self::try_from(value);
            }
        }
        Err(Self::Error::new(proc_macro2::Span::call_site(), "Expected Transform value"))
    }
}
impl TryFrom<syn_args::Arguments> for ModuleArgs {
    type Error = syn::Error;
    fn try_from(value: syn_args::Arguments) -> Result<Self, Self::Error> {
        Self::try_from(&value.0)
    }
}
pub struct ModuleSubObj {
    pub global: def::Option<def::Bool>,
    pub imports: def::Array<def::Expr>,
    pub sub: def::Option<Sub>,
}
#[automatically_derived]
impl ::core::fmt::Debug for ModuleSubObj {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field3_finish(f, "ModuleSubObj", "global", &self.global, "imports", &self.imports, "sub", &&self.sub)
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for ModuleSubObj {}
#[automatically_derived]
impl ::core::cmp::PartialEq for ModuleSubObj {
    #[inline]
    fn eq(&self, other: &ModuleSubObj) -> bool {
        self.global == other.global && self.imports == other.imports && self.sub == other.sub
    }
}
impl TryFrom<&syn_args::Value> for ModuleSubObj {
    type Error = syn::Error;
    fn try_from(v: &syn_args::Value) -> Result<Self, Self::Error> {
        if let syn_args::Value::Object(_) = v {
            return Ok(ModuleSubObj {
                global: syn_args::Transform::new(v, "global").try_into()?,
                imports: syn_args::Transform::new(v, "imports").try_into()?,
                sub: syn_args::Transform::new(v, "sub").try_into()?,
            });
        }
        Err(Self::Error::new(
            proc_macro2::Span::call_site(),
            ::alloc::__export::must_use({
                let res = ::alloc::fmt::format(format_args!("Invalid args try_from {0}", "ModuleSubObj"));
                res
            }),
        ))
    }
}
impl TryFrom<syn_args::Value> for ModuleSubObj {
    type Error = syn::Error;
    fn try_from(v: syn_args::Value) -> Result<Self, Self::Error> {
        ModuleSubObj::try_from(&v)
    }
}
impl syn_args::ArgsParse for ModuleSubObj {
    fn parse(input: &str) -> Result<Self, syn::Error> {
        syn_args::Formal::new().parse(input)?.try_into()
    }
}
impl TryFrom<syn_args::Transform<'_>> for ModuleSubObj {
    type Error = syn::Error;
    fn try_from(value: syn_args::Transform) -> Result<Self, Self::Error> {
        if let syn_args::Value::Object(obj) = value.value {
            if let Some(v) = obj.get(value.key) {
                return v.try_into();
            }
        } else if let syn_args::Value::Array(v) = value.value {
            let index = value.key.parse::<usize>().unwrap();
            if let Some(value) = v.get(index) {
                return Self::try_from(value);
            }
        }
        Err(Self::Error::new(proc_macro2::Span::call_site(), "Expected Transform value"))
    }
}
impl TryFrom<syn_args::Arguments> for ModuleSubObj {
    type Error = syn::Error;
    fn try_from(value: syn_args::Arguments) -> Result<Self, Self::Error> {
        if let syn_args::Value::Array(v) = value.0 {
            if let Some(value) = v.first() {
                return Self::try_from(value);
            }
        }
        Err(Self::Error::new(proc_macro2::Span::call_site(), "Arguments Into T"))
    }
}
pub struct Sub {
    pub value: def::Bool,
}
#[automatically_derived]
impl ::core::fmt::Debug for Sub {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(f, "Sub", "value", &&self.value)
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for Sub {}
#[automatically_derived]
impl ::core::cmp::PartialEq for Sub {
    #[inline]
    fn eq(&self, other: &Sub) -> bool {
        self.value == other.value
    }
}
impl TryFrom<&syn_args::Value> for Sub {
    type Error = syn::Error;
    fn try_from(v: &syn_args::Value) -> Result<Self, Self::Error> {
        if let syn_args::Value::Object(_) = v {
            return Ok(Sub { value: syn_args::Transform::new(v, "value").try_into()? });
        }
        Err(Self::Error::new(
            proc_macro2::Span::call_site(),
            ::alloc::__export::must_use({
                let res = ::alloc::fmt::format(format_args!("Invalid args try_from {0}", "Sub"));
                res
            }),
        ))
    }
}
impl TryFrom<syn_args::Value> for Sub {
    type Error = syn::Error;
    fn try_from(v: syn_args::Value) -> Result<Self, Self::Error> {
        Sub::try_from(&v)
    }
}
impl syn_args::ArgsParse for Sub {
    fn parse(input: &str) -> Result<Self, syn::Error> {
        syn_args::Formal::new().parse(input)?.try_into()
    }
}
impl TryFrom<syn_args::Transform<'_>> for Sub {
    type Error = syn::Error;
    fn try_from(value: syn_args::Transform) -> Result<Self, Self::Error> {
        if let syn_args::Value::Object(obj) = value.value {
            if let Some(v) = obj.get(value.key) {
                return v.try_into();
            }
        } else if let syn_args::Value::Array(v) = value.value {
            let index = value.key.parse::<usize>().unwrap();
            if let Some(value) = v.get(index) {
                return Self::try_from(value);
            }
        }
        Err(Self::Error::new(proc_macro2::Span::call_site(), "Expected Transform value"))
    }
}
impl TryFrom<syn_args::Arguments> for Sub {
    type Error = syn::Error;
    fn try_from(value: syn_args::Arguments) -> Result<Self, Self::Error> {
        if let syn_args::Value::Array(v) = value.0 {
            if let Some(value) = v.first() {
                return Self::try_from(value);
            }
        }
        Err(Self::Error::new(proc_macro2::Span::call_site(), "Arguments Into T"))
    }
}
pub struct SubWrap {
    pub s1: Sub,
    pub s2: Sub,
}
#[automatically_derived]
impl ::core::fmt::Debug for SubWrap {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(f, "SubWrap", "s1", &self.s1, "s2", &&self.s2)
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for SubWrap {}
#[automatically_derived]
impl ::core::cmp::PartialEq for SubWrap {
    #[inline]
    fn eq(&self, other: &SubWrap) -> bool {
        self.s1 == other.s1 && self.s2 == other.s2
    }
}
impl TryFrom<&syn_args::Value> for SubWrap {
    type Error = syn::Error;
    fn try_from(v: &syn_args::Value) -> Result<Self, Self::Error> {
        if let syn_args::Value::Object(_) = v {
            return Ok(SubWrap { s1: syn_args::Transform::new(v, "s1").try_into()?, s2: syn_args::Transform::new(v, "s2").try_into()? });
        }
        Err(Self::Error::new(
            proc_macro2::Span::call_site(),
            ::alloc::__export::must_use({
                let res = ::alloc::fmt::format(format_args!("Invalid args try_from {0}", "SubWrap"));
                res
            }),
        ))
    }
}
impl TryFrom<syn_args::Value> for SubWrap {
    type Error = syn::Error;
    fn try_from(v: syn_args::Value) -> Result<Self, Self::Error> {
        SubWrap::try_from(&v)
    }
}
impl syn_args::ArgsParse for SubWrap {
    fn parse(input: &str) -> Result<Self, syn::Error> {
        syn_args::Formal::new().parse(input)?.try_into()
    }
}
impl TryFrom<syn_args::Transform<'_>> for SubWrap {
    type Error = syn::Error;
    fn try_from(value: syn_args::Transform) -> Result<Self, Self::Error> {
        if let syn_args::Value::Object(obj) = value.value {
            if let Some(v) = obj.get(value.key) {
                return v.try_into();
            }
        } else if let syn_args::Value::Array(v) = value.value {
            let index = value.key.parse::<usize>().unwrap();
            if let Some(value) = v.get(index) {
                return Self::try_from(value);
            }
        }
        Err(Self::Error::new(proc_macro2::Span::call_site(), "Expected Transform value"))
    }
}
impl TryFrom<syn_args::Arguments> for SubWrap {
    type Error = syn::Error;
    fn try_from(value: syn_args::Arguments) -> Result<Self, Self::Error> {
        if let syn_args::Value::Array(v) = value.0 {
            if let Some(value) = v.first() {
                return Self::try_from(value);
            }
        }
        Err(Self::Error::new(proc_macro2::Span::call_site(), "Arguments Into T"))
    }
}
struct T1 {
    pub controllers: def::Option<def::Array<def::Expr>>,
}
#[automatically_derived]
impl ::core::fmt::Debug for T1 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(f, "T1", "controllers", &&self.controllers)
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for T1 {}
#[automatically_derived]
impl ::core::cmp::PartialEq for T1 {
    #[inline]
    fn eq(&self, other: &T1) -> bool {
        self.controllers == other.controllers
    }
}
impl TryFrom<&syn_args::Value> for T1 {
    type Error = syn::Error;
    fn try_from(v: &syn_args::Value) -> Result<Self, Self::Error> {
        if let syn_args::Value::Object(_) = v {
            return Ok(T1 { controllers: syn_args::Transform::new(v, "controllers").try_into()? });
        }
        Err(Self::Error::new(
            proc_macro2::Span::call_site(),
            ::alloc::__export::must_use({
                let res = ::alloc::fmt::format(format_args!("Invalid args try_from {0}", "T1"));
                res
            }),
        ))
    }
}
impl TryFrom<syn_args::Value> for T1 {
    type Error = syn::Error;
    fn try_from(v: syn_args::Value) -> Result<Self, Self::Error> {
        T1::try_from(&v)
    }
}
impl syn_args::ArgsParse for T1 {
    fn parse(input: &str) -> Result<Self, syn::Error> {
        syn_args::Formal::new().parse(input)?.try_into()
    }
}
impl TryFrom<syn_args::Transform<'_>> for T1 {
    type Error = syn::Error;
    fn try_from(value: syn_args::Transform) -> Result<Self, Self::Error> {
        if let syn_args::Value::Object(obj) = value.value {
            if let Some(v) = obj.get(value.key) {
                return v.try_into();
            }
        } else if let syn_args::Value::Array(v) = value.value {
            let index = value.key.parse::<usize>().unwrap();
            if let Some(value) = v.get(index) {
                return Self::try_from(value);
            }
        }
        Err(Self::Error::new(proc_macro2::Span::call_site(), "Expected Transform value"))
    }
}
impl TryFrom<syn_args::Arguments> for T1 {
    type Error = syn::Error;
    fn try_from(value: syn_args::Arguments) -> Result<Self, Self::Error> {
        if let syn_args::Value::Array(v) = value.0 {
            if let Some(value) = v.first() {
                return Self::try_from(value);
            }
        }
        Err(Self::Error::new(proc_macro2::Span::call_site(), "Arguments Into T"))
    }
}
fn test_formal_f1() {
    let f = Formal::new();
    let args = f.parse("F(1, 3)").unwrap();
    {
        ::std::io::_print(format_args!("{0:?}\n", args));
    };
    let res = ModuleArgs::try_from(args).unwrap();
    {
        ::std::io::_print(format_args!("{0:?}\n", res));
    };
    match (&res, &ModuleArgs::F1(def::Int(1), def::Int(3))) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val, &*right_val, ::core::option::Option::None);
            }
        }
    };
}
fn test_formal_f2() {
    let f = Formal::new();
    let args = f.parse("F(1)").unwrap();
    {
        ::std::io::_print(format_args!("{0:?}\n", args));
    };
    let res = ModuleArgs::try_from(args).unwrap();
    {
        ::std::io::_print(format_args!("{0:?}\n", res));
    };
    match (&res, &ModuleArgs::F2(def::Int(1))) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val, &*right_val, ::core::option::Option::None);
            }
        }
    };
}
fn test_formal_f3() {
    let res = ModuleArgs::parse("F(Hello)").unwrap();
    {
        ::std::io::_print(format_args!("{0:?}\n", res));
    };
    match (&res, &ModuleArgs::F3(def::Expr::from("Hello"))) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val, &*right_val, ::core::option::Option::None);
            }
        }
    };
}
fn test_formal_f4() {
    let res = ModuleArgs::parse("F([Ident1, Ident2])").unwrap();
    {
        ::std::io::_print(format_args!("{0:?}\n", res));
    };
    match (
        &res,
        &ModuleArgs::F4(def::Array(<[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([def::Expr::from("Ident1"), def::Expr::from("Ident2")]),
        ))),
    ) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val, &*right_val, ::core::option::Option::None);
            }
        }
    };
}
fn test_formal_f5() {
    let res = ModuleArgs::parse("F({ imports: [Ident1, Ident2] })").unwrap();
    {
        ::std::io::_print(format_args!("{0:?}\n", res));
    };
    match (
        &res,
        &ModuleArgs::F5(ModuleSubObj {
            imports: def::Array(<[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([def::Expr::from("Ident1"), def::Expr::from("Ident2")]),
            )),
            global: def::Option(None),
            sub: def::Option(None),
        }),
    ) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val, &*right_val, ::core::option::Option::None);
            }
        }
    };
}
fn test_formal_f6() {
    let res = ModuleArgs::parse("F([{ imports: [Ident1, Ident2] }, { imports: [Ident3, Ident4] }])").unwrap();
    {
        ::std::io::_print(format_args!("{0:?}\n", res));
    };
    match (
        &res,
        &ModuleArgs::F6(def::Array(<[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                ModuleSubObj {
                    imports: def::Array(<[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([def::Expr::from("Ident1"), def::Expr::from("Ident2")]),
                    )),
                    global: def::Option(None),
                    sub: def::Option(None),
                },
                ModuleSubObj {
                    imports: def::Array(<[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([def::Expr::from("Ident3"), def::Expr::from("Ident4")]),
                    )),
                    global: def::Option(None),
                    sub: def::Option(None),
                },
            ]),
        ))),
    ) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val, &*right_val, ::core::option::Option::None);
            }
        }
    };
}
fn test_formal_f6_2() {
    let res = ModuleArgs::parse("F([{ imports: [Ident1, Ident2], global: true }, { imports: [Ident3, Ident4] }])").unwrap();
    {
        ::std::io::_print(format_args!("{0:?}\n", res));
    };
    match (
        &res,
        &ModuleArgs::F6(def::Array(<[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                ModuleSubObj {
                    imports: def::Array(<[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([def::Expr::from("Ident1"), def::Expr::from("Ident2")]),
                    )),
                    global: def::Option(Some(def::Bool(true))),
                    sub: def::Option(None),
                },
                ModuleSubObj {
                    imports: def::Array(<[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([def::Expr::from("Ident3"), def::Expr::from("Ident4")]),
                    )),
                    global: def::Option(None),
                    sub: def::Option(None),
                },
            ]),
        ))),
    ) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val, &*right_val, ::core::option::Option::None);
            }
        }
    };
}
fn test_formal_f6_3() {
    let res = ModuleArgs::parse("F([{ imports: [Ident1, Ident2], global: true, sub: { value: true } }, { imports: [Ident3, Ident4] }])").unwrap();
    {
        ::std::io::_print(format_args!("{0:?}\n", res));
    };
    match (
        &res,
        &ModuleArgs::F6(def::Array(<[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                ModuleSubObj {
                    imports: def::Array(<[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([def::Expr::from("Ident1"), def::Expr::from("Ident2")]),
                    )),
                    global: def::Option(Some(def::Bool(true))),
                    sub: def::Option(Some(Sub { value: def::Bool(true) })),
                },
                ModuleSubObj {
                    imports: def::Array(<[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([def::Expr::from("Ident3"), def::Expr::from("Ident4")]),
                    )),
                    global: def::Option(None),
                    sub: def::Option(None),
                },
            ]),
        ))),
    ) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val, &*right_val, ::core::option::Option::None);
            }
        }
    };
}
fn test_formal_f7() {
    let res = ModuleArgs::parse("F({ s1: { value: false }, s2: { value: true } })").unwrap();
    {
        ::std::io::_print(format_args!("{0:?}\n", res));
    };
    match (&res, &ModuleArgs::F7(SubWrap { s1: Sub { value: def::Bool(false) }, s2: Sub { value: def::Bool(true) } })) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val, &*right_val, ::core::option::Option::None);
            }
        }
    };
}
fn test_tokens_formal_f7() {
    let res = ModuleArgs::parse("F({ s1: { value: false }, s2: { value: true } })").unwrap();
    {
        ::std::io::_print(format_args!("{0:?}\n", res));
    };
    match (&res, &ModuleArgs::F7(SubWrap { s1: Sub { value: def::Bool(false) }, s2: Sub { value: def::Bool(true) } })) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val, &*right_val, ::core::option::Option::None);
            }
        }
    };
}
fn test_formal_f8() {
    let res = ModuleArgs::parse("F({ controllers: [Ident1, Ident2] })").unwrap();
    {
        ::std::io::_print(format_args!("{0:?}\n", res));
    };
    match (
        &res,
        &ModuleArgs::F8(def::Option(Some(T1 {
            controllers: def::Option(Some(def::Array(<[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([def::Expr::from("Ident1"), def::Expr::from("Ident2")]),
            )))),
        }))),
    ) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val, &*right_val, ::core::option::Option::None);
            }
        }
    };
    let res = ModuleArgs::parse("F()").unwrap();
    {
        ::std::io::_print(format_args!("{0:?}\n", res));
    };
    match (&res, &ModuleArgs::F8(def::Option(None))) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val, &*right_val, ::core::option::Option::None);
            }
        }
    };
}
fn test_value_p1() {
    let f = Formal::new();
    let args = f.parse("F(1, { a:1, b:2 })").unwrap();
    {
        ::std::io::_print(format_args!("{0:?}\n", args));
    };
    match (
        &args.0,
        &Value::Array(def::Array(<[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                Value::Int(def::Int(1)),
                Value::Object(def::Object(
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([("a".to_string(), Value::Int(def::Int(1))), ("b".to_string(), Value::Int(def::Int(2)))]),
                    )
                    .into_iter()
                    .collect(),
                )),
            ]),
        ))),
    ) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val, &*right_val, ::core::option::Option::None);
            }
        }
    };
}
fn test_into_object_p1() {
    let f = Formal::new();
    let args = f.parse("F({ imports: [Ident1::register(), Ident2] })").unwrap();
    {
        ::std::io::_print(format_args!("{0:?}\n", args));
    };
    let res = ModuleSubObj::try_from(args).unwrap();
    match (
        &res,
        &ModuleSubObj {
            imports: def::Array(<[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([def::Expr::from("Ident1::register ()"), def::Expr::from("Ident2")]),
            )),
            global: def::Option(None),
            sub: def::Option(None),
        },
    ) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(kind, &*left_val, &*right_val, ::core::option::Option::None);
            }
        }
    };
}
fn main() {
    test_formal_f1();
    test_formal_f2();
    test_formal_f3();
    test_formal_f4();
    test_formal_f5();
    test_formal_f6();
    test_formal_f6_2();
    test_formal_f6_3();
    test_formal_f7();
    test_tokens_formal_f7();
    test_formal_f8();
    test_into_object_p1();
}
extern crate alloc;
