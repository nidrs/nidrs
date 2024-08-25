#![feature(print_internals)]
#![feature(structural_match)]
#![feature(core_intrinsics)]
#![feature(panic_internals)]
#![feature(rustc_attrs)]
#![feature(alloc)]
#![feature(fmt_helpers_for_derive)]
#![allow(warnings, unused)]
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use syn::Error;
use syn_args::{def, derive::ArgsParse, ArgsParse, Formal};
pub enum ModuleArgs {
    F1(def::Int, def::Int),
    F2(def::Int),
    F3(def::Ident),
    F4(def::Array<def::Ident>),
    F5(ModuleSubObj),
    F6(def::Array<ModuleSubObj>),
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
                (ModuleArgs::F1(__self_0, __self_1), ModuleArgs::F1(__arg1_0, __arg1_1)) => *__self_0 == *__arg1_0 && *__self_1 == *__arg1_1,
                (ModuleArgs::F2(__self_0), ModuleArgs::F2(__arg1_0)) => *__self_0 == *__arg1_0,
                (ModuleArgs::F3(__self_0), ModuleArgs::F3(__arg1_0)) => *__self_0 == *__arg1_0,
                (ModuleArgs::F4(__self_0), ModuleArgs::F4(__arg1_0)) => *__self_0 == *__arg1_0,
                (ModuleArgs::F5(__self_0), ModuleArgs::F5(__arg1_0)) => *__self_0 == *__arg1_0,
                (ModuleArgs::F6(__self_0), ModuleArgs::F6(__arg1_0)) => *__self_0 == *__arg1_0,
                _ => unsafe { ::core::intrinsics::unreachable() },
            }
    }
}
impl TryFrom<&syn_args::Value> for ModuleArgs {
    type Error = syn::Error;
    fn try_from(v: &syn_args::Value) -> Result<Self, Self::Error> {
        if let syn_args::Value::Array(v) = v {
            if let Ok(rt) = syn_args::utils::ewc::<_, _, anyhow::Error>(|| {
                Ok(ModuleArgs::F1(syn_args::utils::otr(v.get(0usize))?.try_into()?, syn_args::utils::otr(v.get(1usize))?.try_into()?))
            }) {
                return Ok(rt);
            }
            if let Ok(rt) = syn_args::utils::ewc::<_, _, anyhow::Error>(|| Ok(ModuleArgs::F2(syn_args::utils::otr(v.get(0usize))?.try_into()?))) {
                return Ok(rt);
            }
            if let Ok(rt) = syn_args::utils::ewc::<_, _, anyhow::Error>(|| Ok(ModuleArgs::F3(syn_args::utils::otr(v.get(0usize))?.try_into()?))) {
                return Ok(rt);
            }
            if let Ok(rt) = syn_args::utils::ewc::<_, _, anyhow::Error>(|| Ok(ModuleArgs::F4(syn_args::utils::otr(v.get(0usize))?.try_into()?))) {
                return Ok(rt);
            }
            if let Ok(rt) = syn_args::utils::ewc::<_, _, anyhow::Error>(|| Ok(ModuleArgs::F5(syn_args::utils::otr(v.get(0usize))?.try_into()?))) {
                return Ok(rt);
            }
            if let Ok(rt) = syn_args::utils::ewc::<_, _, anyhow::Error>(|| Ok(ModuleArgs::F6(syn_args::utils::otr(v.get(0usize))?.try_into()?))) {
                return Ok(rt);
            }
        }
        Err(Self::Error::new(proc_macro2::Span::call_site(), "Invalid args"))
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
                return Ok(v.try_into()?);
            }
        }
        Err(Self::Error::new(proc_macro2::Span::call_site(), "Expected SubWrap"))
    }
}
pub struct ModuleSubObj {
    pub imports: def::Array<def::Ident>,
}
#[automatically_derived]
impl ::core::fmt::Debug for ModuleSubObj {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(f, "ModuleSubObj", "imports", &&self.imports)
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for ModuleSubObj {}
#[automatically_derived]
impl ::core::cmp::PartialEq for ModuleSubObj {
    #[inline]
    fn eq(&self, other: &ModuleSubObj) -> bool {
        self.imports == other.imports
    }
}
impl TryFrom<&syn_args::Value> for ModuleSubObj {
    type Error = syn::Error;
    fn try_from(v: &syn_args::Value) -> Result<Self, Self::Error> {
        if let syn_args::Value::Object(_) = v {
            return Ok(ModuleSubObj { imports: syn_args::Transform::new(v, "imports").try_into()? });
        }
        Err(Self::Error::new(proc_macro2::Span::call_site(), "Invalid args"))
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
                return Ok(v.try_into()?);
            }
        }
        Err(Self::Error::new(proc_macro2::Span::call_site(), "Expected SubWrap"))
    }
}
fn test_formal_f1() {
    let f = Formal::new();
    let args = f.parse("F(1, 3)").unwrap();
    {
        ::std::io::_print(format_args!("{0:?}\n", args));
    };
    let res = ModuleArgs::try_from(&args).unwrap();
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
    let res = ModuleArgs::try_from(&args).unwrap();
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
    match (&res, &ModuleArgs::F3(def::Ident("Hello".to_string()))) {
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
            ::alloc::boxed::Box::new([def::Ident("Ident1".to_string()), def::Ident("Ident2".to_string())]),
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
                ::alloc::boxed::Box::new([def::Ident("Ident1".to_string()), def::Ident("Ident2".to_string())]),
            )),
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
                        ::alloc::boxed::Box::new([def::Ident("Ident1".to_string()), def::Ident("Ident2".to_string())]),
                    )),
                },
                ModuleSubObj {
                    imports: def::Array(<[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([def::Ident("Ident3".to_string()), def::Ident("Ident4".to_string())]),
                    )),
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
fn main() {
    test_formal_f1();
    test_formal_f2();
    test_formal_f3();
    test_formal_f4();
    test_formal_f5();
    test_formal_f6();
}
extern crate alloc;
