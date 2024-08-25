mod macro_args;
mod traits;
mod transform;

pub use macro_args::*;
pub use syn_args_derive as derive;
pub use traits::*;
pub use transform::*;

#[cfg(test)]
mod tests {

    use syn::Error;
    use traits::ArgsParse;
    use utils::{ewc, otr};

    use super::*;

    // #[args(Module)]

    #[derive(Debug, PartialEq)]
    pub enum ModuleArgs {
        F1(def::Int, def::Int),
        F2(def::Int),
        F3(def::Ident),
        F4(def::Array<def::Ident>),
        F5(ModuleSubObj),
        F6(def::Array<ModuleSubObj>),
        F7(SubWrap),
    }

    impl TryFrom<&Value> for ModuleArgs {
        type Error = Error;
        fn try_from(v: &Value) -> Result<Self, Error> {
            if let Value::Array(args) = v {
                let r = ewc::<_, _, anyhow::Error>(|| Ok(ModuleArgs::F1(otr(args.first())?.try_into()?, otr(args.get(1))?.try_into()?)));
                if let Ok(rt) = r {
                    return Ok(rt);
                }

                let r = ewc::<_, _, anyhow::Error>(|| Ok(ModuleArgs::F2(otr(args.first())?.try_into()?)));
                if let Ok(rt) = r {
                    return Ok(rt);
                }

                let r = ewc::<_, _, anyhow::Error>(|| Ok(ModuleArgs::F3(otr(args.first())?.try_into()?)));
                if let Ok(rt) = r {
                    return Ok(rt);
                }

                let r = ewc::<_, _, anyhow::Error>(|| Ok(ModuleArgs::F4(otr(args.first())?.try_into()?)));
                if let Ok(rt) = r {
                    return Ok(rt);
                }

                let r = ewc::<_, _, anyhow::Error>(|| Ok(ModuleArgs::F5(otr(args.first())?.try_into()?)));
                if let Ok(rt) = r {
                    return Ok(rt);
                }

                let r = ewc::<_, _, anyhow::Error>(|| Ok(ModuleArgs::F6(otr(args.first())?.try_into()?)));
                if let Ok(rt) = r {
                    return Ok(rt);
                }
                let r = ewc::<_, _, anyhow::Error>(|| Ok(ModuleArgs::F7(otr(args.first())?.try_into()?)));
                if let Ok(rt) = r {
                    return Ok(rt);
                }
            }

            Err(Error::new(proc_macro2::Span::call_site(), "Invalid args"))
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

    // #[args_object]
    #[derive(Debug, PartialEq)]
    pub struct ModuleSubObj {
        pub global: def::Option<def::Bool>,
        pub imports: def::Array<def::Ident>,
        pub sub: def::Option<Sub>,
    }

    impl TryFrom<&Value> for ModuleSubObj {
        type Error = Error;

        fn try_from(value: &Value) -> Result<Self, Self::Error> {
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
            }

            Err(Error::new(proc_macro2::Span::call_site(), "Expected SubWrap"))
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

        let res = ModuleArgs::try_from(&args).unwrap();
        println!("{:?}", res);

        assert_eq!(res, ModuleArgs::F1(def::Int(1), def::Int(3)));
    }

    #[test]
    fn test_formal_f2() {
        let f = Formal::new();

        let args = f.parse("F(1)").unwrap();
        // let args = f.parse("F(Hello)").unwrap();
        println!("{:?}", args);

        let res = ModuleArgs::try_from(&args).unwrap();
        println!("{:?}", res);

        assert_eq!(res, ModuleArgs::F2(def::Int(1)));
    }

    #[test]
    fn test_formal_f3() {
        let res = ModuleArgs::parse("F(Hello)").unwrap();
        println!("{:?}", res);

        assert_eq!(res, ModuleArgs::F3(def::Ident("Hello".to_string())));
    }

    #[test]
    fn test_formal_f4() {
        let res = ModuleArgs::parse("F([Ident1, Ident2])").unwrap();
        println!("{:?}", res);

        assert_eq!(res, ModuleArgs::F4(def::Array(vec![def::Ident("Ident1".to_string()), def::Ident("Ident2".to_string())])));
    }

    #[test]
    fn test_formal_f5() {
        let res = ModuleArgs::parse("F({ imports: [Ident1, Ident2] })").unwrap();
        println!("{:?}", res);

        assert_eq!(
            res,
            ModuleArgs::F5(ModuleSubObj {
                imports: def::Array(vec![def::Ident("Ident1".to_string()), def::Ident("Ident2".to_string())]),
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
                    imports: def::Array(vec![def::Ident("Ident1".to_string()), def::Ident("Ident2".to_string())]),
                    global: def::Option(None),
                    sub: def::Option(None)
                },
                ModuleSubObj {
                    imports: def::Array(vec![def::Ident("Ident3".to_string()), def::Ident("Ident4".to_string())]),
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
                    imports: def::Array(vec![def::Ident("Ident1".to_string()), def::Ident("Ident2".to_string())]),
                    global: def::Option(Some(Box::new(def::Bool(true)))),
                    sub: def::Option(None)
                },
                ModuleSubObj {
                    imports: def::Array(vec![def::Ident("Ident3".to_string()), def::Ident("Ident4".to_string())]),
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
                    imports: def::Array(vec![def::Ident("Ident1".to_string()), def::Ident("Ident2".to_string())]),
                    global: def::Option(Some(Box::new(def::Bool(true)))),
                    sub: def::Option(Some(Box::new(Sub { value: def::Bool(true) })))
                },
                ModuleSubObj {
                    imports: def::Array(vec![def::Ident("Ident3".to_string()), def::Ident("Ident4".to_string())]),
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
}
