mod macro_args;
mod syn_args;
mod traits;
mod transform;

pub use anyhow;
pub use macro_args::*;
pub use syn_args::*;
pub use syn_args_derive as derive;
pub use traits::*;
pub use transform::*;

#[cfg(test)]
mod tests {

    use syn::Error;
    use traits::ArgsParse;
    use utils::ewc;

    use super::*;

    #[derive(Debug, PartialEq)]
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

    impl TryFrom<&Value> for ModuleArgs {
        type Error = Error;
        fn try_from(v: &Value) -> Result<Self, Error> {
            let mut err = Error::new(proc_macro2::Span::call_site(), "Expected ModuleArgs");
            if let Value::Array(args) = v {
                match ewc::<_, _, syn::Error>(|| Ok(ModuleArgs::F1(Transform::new(v, "0").try_into()?, Transform::new(v, "1").try_into()?))) {
                    Ok(rt) => return Ok(rt),
                    Err(e) => err = e,
                }

                match Transform::new(v, "0").try_into() {
                    Ok(rt) => return Ok(ModuleArgs::F2(rt)),
                    Err(e) => err = e,
                }

                match Transform::new(v, "0").try_into() {
                    Ok(rt) => return Ok(ModuleArgs::F3(rt)),
                    Err(e) => err = e,
                }

                match Transform::new(v, "0").try_into() {
                    Ok(rt) => return Ok(ModuleArgs::F4(rt)),
                    Err(e) => err = e,
                }

                match Transform::new(v, "0").try_into() {
                    Ok(rt) => return Ok(ModuleArgs::F5(rt)),
                    Err(e) => err = e,
                }

                match Transform::new(v, "0").try_into() {
                    Ok(rt) => return Ok(ModuleArgs::F6(rt)),
                    Err(e) => err = e,
                }

                match Transform::new(v, "0").try_into() {
                    Ok(rt) => return Ok(ModuleArgs::F7(rt)),
                    Err(e) => err = e,
                }

                match Transform::new(v, "0").try_into() {
                    Ok(rt) => return Ok(ModuleArgs::F8(rt)),
                    Err(e) => err = e,
                }
            }

            Err(err)
        }
    }

    impl TryFrom<Value> for ModuleArgs {
        type Error = Error;
        fn try_from(v: Value) -> Result<Self, Error> {
            ModuleArgs::try_from(&v)
        }
    }

    impl ArgsParse for ModuleArgs {
        fn parse(input: &str) -> Result<Self, Error> {
            Formal::new().parse(input)?.try_into()
        }
    }

    impl TryFrom<Arguments> for ModuleArgs {
        type Error = Error;

        fn try_from(value: Arguments) -> Result<Self, Self::Error> {
            Self::try_from(&value.0)
        }
    }

    impl TryFrom<Transform<'_>> for ModuleArgs {
        type Error = Error;

        fn try_from(value: Transform) -> Result<Self, Self::Error> {
            if let Value::Object(obj) = value.value {
                if let Some(v) = obj.get(value.key) {
                    return v.try_into();
                }
            }

            Err(Error::new(proc_macro2::Span::call_site(), "Expected ModuleArgs"))
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct ModuleSubObj {
        pub global: def::Option<def::Bool>,
        pub imports: def::Array<def::Expr>,
        pub sub: def::Option<Sub>,
    }

    impl TryFrom<&Value> for ModuleSubObj {
        type Error = Error;

        fn try_from(value: &Value) -> Result<Self, Self::Error> {
            println!("ModuleSubObj： {:?}", value);
            if let Value::Object(_) = value {
                return Ok(ModuleSubObj {
                    imports: Transform::new(value, "imports").try_into()?,
                    global: Transform::new(value, "global").try_into()?,
                    sub: Transform::new(value, "sub").try_into()?,
                });
            }

            Err(Error::new(proc_macro2::Span::call_site(), "Expected ModuleSubObj"))
        }
    }

    impl TryFrom<Arguments> for ModuleSubObj {
        type Error = Error;

        fn try_from(value: Arguments) -> Result<Self, Self::Error> {
            if let Value::Array(v) = value.0 {
                if let Some(value) = v.first() {
                    return Self::try_from(value);
                }
            }
            Err(Error::new(proc_macro2::Span::call_site(), "Arguments ModuleSubObj"))
        }
    }

    impl TryFrom<Transform<'_>> for ModuleSubObj {
        type Error = Error;

        fn try_from(value: Transform) -> Result<Self, Self::Error> {
            if let Value::Object(obj) = value.value {
                if let Some(v) = obj.get(value.key) {
                    return v.try_into();
                }
            } else if let Value::Array(v) = value.value {
                let index = value.key.parse::<usize>().unwrap();
                if let Some(value) = v.get(index) {
                    return Self::try_from(value);
                }
            }

            Err(Error::new(proc_macro2::Span::call_site(), "Expected ModuleSubObj"))
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct Sub {
        pub value: def::Bool,
    }

    impl TryFrom<&Value> for Sub {
        type Error = Error;

        fn try_from(value: &Value) -> Result<Self, Self::Error> {
            if let Value::Object(_) = value {
                return Ok(Sub { value: Transform::new(value, "value").try_into()? });
            }

            Err(Error::new(proc_macro2::Span::call_site(), "Expected Sub"))
        }
    }

    impl TryFrom<Value> for Sub {
        type Error = Error;

        fn try_from(value: Value) -> Result<Self, Self::Error> {
            Sub::try_from(&value)
        }
    }

    impl TryFrom<Transform<'_>> for Sub {
        type Error = Error;

        fn try_from(value: Transform) -> Result<Self, Self::Error> {
            if let Value::Object(obj) = value.value {
                if let Some(v) = obj.get(value.key) {
                    return v.try_into();
                }
            }

            Err(Error::new(proc_macro2::Span::call_site(), "Expected SubWrap"))
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct SubWrap {
        pub s1: Sub,
        pub s2: Sub,
    }

    impl TryFrom<&Value> for SubWrap {
        type Error = Error;

        fn try_from(value: &Value) -> Result<Self, Self::Error> {
            if let Value::Object(_) = value {
                return Ok(SubWrap { s1: Transform::new(value, "s1").try_into()?, s2: Transform::new(value, "s2").try_into()? });
            }

            Err(Error::new(proc_macro2::Span::call_site(), "Expected SubWrap"))
        }
    }

    impl TryFrom<Value> for SubWrap {
        type Error = Error;

        fn try_from(value: Value) -> Result<Self, Self::Error> {
            SubWrap::try_from(&value)
        }
    }

    impl TryFrom<Transform<'_>> for SubWrap {
        type Error = Error;

        fn try_from(value: Transform) -> Result<Self, Self::Error> {
            if let Value::Object(obj) = value.value {
                if let Some(v) = obj.get(value.key) {
                    return v.try_into();
                }
            } else if let Value::Array(v) = value.value {
                let index = value.key.parse::<usize>().unwrap();
                if let Some(value) = v.get(index) {
                    return Self::try_from(value);
                }
            }

            Err(Error::new(proc_macro2::Span::call_site(), "Expected SubWrap"))
        }
    }

    #[derive(Debug, PartialEq)]
    struct T1 {
        pub controllers: def::Option<def::Array<def::Expr>>,
    }

    impl TryFrom<&Value> for T1 {
        type Error = Error;

        fn try_from(value: &Value) -> Result<Self, Self::Error> {
            if let Value::Object(_) = value {
                return Ok(T1 { controllers: Transform::new(value, "controllers").try_into()? });
            }

            Err(Error::new(proc_macro2::Span::call_site(), "Expected T1"))
        }
    }

    impl TryFrom<Transform<'_>> for T1 {
        type Error = Error;

        fn try_from(value: Transform<'_>) -> Result<Self, Self::Error> {
            println!("{:?} {:?}", value.key, value.value);
            if let Value::Object(obj) = value.value {
                if let Some(v) = obj.get(value.key) {
                    return v.try_into();
                }
            } else if let Value::Array(v) = value.value {
                let index = value.key.parse::<usize>().unwrap();
                if let Some(value) = v.get(index) {
                    return Self::try_from(value);
                }
            }

            Err(Error::new(proc_macro2::Span::call_site(), "Expected T1"))
        }
    }

    #[test]
    fn test_formal_f1() {
        let f = Formal::new();

        // fm.parse("F(Object{ a: Int, b: Optional(Int) }, Array(Int))");
        let args = f.parse("F(1, 3)").unwrap();
        // let args = f.parse("F(1)").unwrap();
        // let args = f.parse("F(Hello)").unwrap();
        println!("{:?}", args);

        let res = ModuleArgs::try_from(args).unwrap();
        println!("{:?}", res);

        assert_eq!(res, ModuleArgs::F1(def::Int(1), def::Int(3)));
    }

    #[test]
    fn test_formal_f2() {
        let f = Formal::new();

        let args = f.parse("F(1)").unwrap();
        // let args = f.parse("F(Hello)").unwrap();
        println!("{:?}", args);

        let res = ModuleArgs::try_from(args).unwrap();
        println!("{:?}", res);

        assert_eq!(res, ModuleArgs::F2(def::Int(1)));
    }

    #[test]
    fn test_formal_f3() {
        let res = ModuleArgs::parse("F(Hello)").unwrap();
        println!("{:?}", res);

        assert_eq!(res, ModuleArgs::F3(def::Expr::from("Hello")));
    }

    #[test]
    fn test_formal_f4() {
        let res = ModuleArgs::parse("F([Ident1, Ident2])").unwrap();
        println!("{:?}", res);

        assert_eq!(res, ModuleArgs::F4(def::Array(vec![def::Expr::from("Ident1"), def::Expr::from("Ident2")])));
    }

    #[test]
    fn test_formal_f5() {
        let res = ModuleArgs::parse("F({ imports: [Ident1, Ident2] })").unwrap();
        println!("{:?}", res);

        assert_eq!(
            res,
            ModuleArgs::F5(ModuleSubObj {
                imports: def::Array(vec![def::Expr::from("Ident1"), def::Expr::from("Ident2")]),
                global: def::Option(None),
                sub: def::Option(None)
            })
        );
    }

    #[test]
    fn test_formal_f6() {
        let res = ModuleArgs::parse("F([{ imports: [Ident1, Ident2] }, { imports: [Ident3, Ident4] }])").unwrap();
        println!("{:?}", res);

        assert_eq!(
            res,
            ModuleArgs::F6(def::Array(vec![
                ModuleSubObj {
                    imports: def::Array(vec![def::Expr::from("Ident1"), def::Expr::from("Ident2")]),
                    global: def::Option(None),
                    sub: def::Option(None)
                },
                ModuleSubObj {
                    imports: def::Array(vec![def::Expr::from("Ident3"), def::Expr::from("Ident4")]),
                    global: def::Option(None),
                    sub: def::Option(None)
                }
            ]))
        );
    }

    #[test]
    fn test_formal_f6_2() {
        let res = ModuleArgs::parse("F([{ imports: [Ident1, Ident2], global: true }, { imports: [Ident3, Ident4] }])").unwrap();
        println!("{:?}", res);

        assert_eq!(
            res,
            ModuleArgs::F6(def::Array(vec![
                ModuleSubObj {
                    imports: def::Array(vec![def::Expr::from("Ident1"), def::Expr::from("Ident2")]),
                    global: def::Option(Some(def::Bool(true))),
                    sub: def::Option(None)
                },
                ModuleSubObj {
                    imports: def::Array(vec![def::Expr::from("Ident3"), def::Expr::from("Ident4")]),
                    global: def::Option(None),
                    sub: def::Option(None)
                }
            ]))
        );
    }

    #[test]
    fn test_formal_f6_3() {
        let res = ModuleArgs::parse("F([{ imports: [Ident1, Ident2], global: true, sub: { value: true } }, { imports: [Ident3, Ident4] }])").unwrap();
        println!("{:?}", res);

        assert_eq!(
            res,
            ModuleArgs::F6(def::Array(vec![
                ModuleSubObj {
                    imports: def::Array(vec![def::Expr::from("Ident1"), def::Expr::from("Ident2")]),
                    global: def::Option(Some(def::Bool(true))),
                    sub: def::Option(Some(Sub { value: def::Bool(true) }))
                },
                ModuleSubObj {
                    imports: def::Array(vec![def::Expr::from("Ident3"), def::Expr::from("Ident4")]),
                    global: def::Option(None),
                    sub: def::Option(None)
                }
            ]))
        );
    }
    #[test]
    fn test_formal_f7() {
        let res = ModuleArgs::parse("F({ s1: { value: false }, s2: { value: true } })").unwrap();
        println!("{:?}", res);

        assert_eq!(res, ModuleArgs::F7(SubWrap { s1: Sub { value: def::Bool(false) }, s2: Sub { value: def::Bool(true) } }));
    }

    #[test]
    fn test_tokens_formal_f7() {
        let res = ModuleArgs::parse("F({ s1: { value: false }, s2: { value: true } })").unwrap();
        println!("{:?}", res);

        assert_eq!(res, ModuleArgs::F7(SubWrap { s1: Sub { value: def::Bool(false) }, s2: Sub { value: def::Bool(true) } }));
    }

    #[test]
    fn test_formal_f8() {
        let res = ModuleArgs::parse("F({ controllers: [Ident1, Ident2] })").unwrap();
        println!("{:?}", res);

        assert_eq!(
            res,
            ModuleArgs::F8(def::Option(Some(T1 {
                controllers: def::Option(Some(def::Array(vec![def::Expr::from("Ident1"), def::Expr::from("Ident2")])))
            })))
        );

        let res = ModuleArgs::parse("F()").unwrap();
        println!("{:?}", res);

        assert_eq!(res, ModuleArgs::F8(def::Option(None)));
    }

    // #[test]
    fn test_value_p1() {
        let f = Formal::new();

        let args = f.parse("F(1, { a:1, b:2 })").unwrap();
        println!("{:?}", args);

        assert_eq!(
            args.0,
            Value::Array(def::Array(vec![
                Value::Int(def::Int(1)),
                Value::Object(def::Object(
                    vec![("a".to_string(), Value::Int(def::Int(1))), ("b".to_string(), Value::Int(def::Int(2)))].into_iter().collect()
                ))
            ]))
        );
    }

    #[test]
    fn test_into_object_p1() {
        let f = Formal::new();
        let args = f.parse("F({ imports: [Ident1::register(), Ident2] })").unwrap();
        println!("{:?}", args);
        let res = ModuleSubObj::try_from(args).unwrap();

        assert_eq!(
            res,
            ModuleSubObj {
                imports: def::Array(vec![def::Expr::from("Ident1::register ()"), def::Expr::from("Ident2")]),
                global: def::Option(None),
                sub: def::Option(None)
            }
        );
    }
}
