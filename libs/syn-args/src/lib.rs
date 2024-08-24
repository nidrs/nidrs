mod macro_args;
mod traits;
pub use macro_args::*;
pub use syn_args_derive as derive;
pub use traits::*;

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

    // #[args_object]
    #[derive(Debug, PartialEq)]
    pub struct ModuleSubObj {
        pub imports: def::Array<def::Ident>,
        // pub interceptors: def::Array<def::Ident>,
        // pub controllers: def::Array<def::Ident>,
        // pub services: def::Array<def::Ident>,
        // pub exports: def::Array<def::Ident>,
    }

    impl TryFrom<&Value> for ModuleSubObj {
        type Error = Error;

        fn try_from(value: &Value) -> Result<Self, Self::Error> {
            if let Value::Object(v) = value {
                return Ok(ModuleSubObj { imports: otr(v.get("imports"))?.try_into()? });
            }

            Err(Error::new(proc_macro2::Span::call_site(), "Expected ModuleSubObj"))
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
            ModuleArgs::F5(ModuleSubObj { imports: def::Array(vec![def::Ident("Ident1".to_string()), def::Ident("Ident2".to_string())]) })
        );
    }

    #[test]
    fn test_formal_f6() {
        let res = ModuleArgs::parse("F([{ imports: [Ident1, Ident2] }, { imports: [Ident3, Ident4] }])").unwrap();
        println!("{:?}", res);

        assert_eq!(
            res,
            ModuleArgs::F6(def::Array(vec![
                ModuleSubObj { imports: def::Array(vec![def::Ident("Ident1".to_string()), def::Ident("Ident2".to_string())]) },
                ModuleSubObj { imports: def::Array(vec![def::Ident("Ident3".to_string()), def::Ident("Ident4".to_string())]) }
            ]))
        );
    }
}
