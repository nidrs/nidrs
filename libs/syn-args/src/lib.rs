mod macro_args;

#[cfg(test)]
mod tests {
    use def::Object;
    use syn::Error;
    use utils::{ewc, otr};

    use crate::macro_args::*;

    // #[args(Module)]

    #[derive(Debug, PartialEq)]
    pub enum ModuleArgs {
        F1(def::Int, def::Int),
        F2(def::Int),
        F3(def::Ident),
        F4(def::Array<def::Ident>),
        F5(def::Object<ModuleSubObj>),
        F6(def::Array<def::Object<ModuleSubObj>>),
    }

    impl ModuleArgs {
        pub fn parse(args: Vec<Value>) -> Result<Self, Error> {
            let r: Result<ModuleArgs, anyhow::Error> = ewc(|| Ok(ModuleArgs::F1(otr(args.first())?.try_into()?, otr(args.get(1))?.try_into()?)));
            if let Ok(rt) = r {
                return Ok(rt);
            }

            let r: Result<ModuleArgs, anyhow::Error> = ewc(|| Ok(ModuleArgs::F2(otr(args.first())?.try_into()?)));
            if let Ok(rt) = r {
                return Ok(rt);
            }

            let r: Result<ModuleArgs, anyhow::Error> = ewc(|| Ok(ModuleArgs::F3(otr(args.first())?.try_into()?)));
            if let Ok(rt) = r {
                return Ok(rt);
            }

            let r: Result<ModuleArgs, anyhow::Error> = ewc(|| Ok(ModuleArgs::F4(otr(args.first())?.try_into()?)));
            if let Ok(rt) = r {
                return Ok(rt);
            }

            let r: Result<ModuleArgs, anyhow::Error> = ewc(|| Ok(ModuleArgs::F5(otr(args.first())?.try_into()?)));
            if let Ok(rt) = r {
                return Ok(rt);
            }

            let r: Result<ModuleArgs, anyhow::Error> = ewc(|| Ok(ModuleArgs::F6(otr(args.first())?.try_into()?)));
            if let Ok(rt) = r {
                return Ok(rt);
            }

            Err(Error::new(proc_macro2::Span::call_site(), "Invalid args"))
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

    impl TryFrom<&Value> for Object<ModuleSubObj> {
        type Error = Error;

        fn try_from(value: &Value) -> Result<Self, Self::Error> {
            match value {
                Value::Object(obj) => {
                    let imports = obj.0.get("imports").ok_or(Error::new(proc_macro2::Span::call_site(), "Expected imports"))?.try_into()?;
                    Ok(Object(ModuleSubObj { imports }))
                }
                _ => Err(Error::new(proc_macro2::Span::call_site(), "Expected ModuleSubObj")),
            }
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

        let res = ModuleArgs::parse(args).unwrap();
        println!("{:?}", res);

        assert_eq!(res, ModuleArgs::F1(def::Int(1), def::Int(3)));
    }

    #[test]
    fn test_formal_f2() {
        let f = Formal::new();

        let args = f.parse("F(1)").unwrap();
        // let args = f.parse("F(Hello)").unwrap();
        println!("{:?}", args);

        let res = ModuleArgs::parse(args).unwrap();
        println!("{:?}", res);

        assert_eq!(res, ModuleArgs::F2(def::Int(1)));
    }

    #[test]
    fn test_formal_f3() {
        let f = Formal::new();

        let args = f.parse("F(Hello)").unwrap();
        println!("{:?}", args);

        let res = ModuleArgs::parse(args).unwrap();
        println!("{:?}", res);

        assert_eq!(res, ModuleArgs::F3(def::Ident("Hello".to_string())));
    }

    #[test]
    fn test_formal_f4() {
        let f = Formal::new();

        let args = f.parse("F([Ident1, Ident2])").unwrap();
        println!("{:?}", args);

        let res = ModuleArgs::parse(args).unwrap();
        println!("{:?}", res);

        assert_eq!(res, ModuleArgs::F4(def::Array(vec![def::Ident("Ident1".to_string()), def::Ident("Ident2".to_string())])));
    }

    #[test]
    fn test_formal_f5() {
        let f = Formal::new();

        let args = f.parse("F({ imports: [Ident1, Ident2] })").unwrap();
        println!("{:?}", args);

        let res = ModuleArgs::parse(args).unwrap();
        println!("{:?}", res);

        assert_eq!(
            res,
            ModuleArgs::F5(def::Object(ModuleSubObj {
                imports: def::Array(vec![def::Ident("Ident1".to_string()), def::Ident("Ident2".to_string())])
            }))
        );
    }

    #[test]
    fn test_formal_f6() {
        let f = Formal::new();

        let args = f.parse("F([{ imports: [Ident1, Ident2] }, { imports: [Ident3, Ident4] }])").unwrap();
        println!("{:?}", args);

        let res = ModuleArgs::parse(args).unwrap();
        println!("{:?}", res);

        assert_eq!(
            res,
            ModuleArgs::F6(def::Array(vec![
                def::Object(ModuleSubObj { imports: def::Array(vec![def::Ident("Ident1".to_string()), def::Ident("Ident2".to_string())]) }),
                def::Object(ModuleSubObj { imports: def::Array(vec![def::Ident("Ident3".to_string()), def::Ident("Ident4".to_string())]) })
            ]))
        );
    }
}
