mod macro_args;

#[cfg(test)]
mod tests {
    use syn::Error;

    use crate::macro_args::*;

    // #[args(Module)]

    #[derive(Debug, PartialEq)]
    pub enum ModuleArgs {
        F1(def::Int, def::Int),
        F2(def::Int),
        F3(def::Ident),
        F4(def::Array<def::Ident>),
    }

    impl ModuleArgs {
        pub fn parse(args: Vec<Value>) -> Result<Self, Error> {
            let r: Result<ModuleArgs, anyhow::Error> = ewc(|| Ok(ModuleArgs::F1(otr(args.get(0))?.try_into()?, otr(args.get(1))?.try_into()?)));
            if let Ok(rt) = r {
                return Ok(rt);
            }

            let r: Result<ModuleArgs, anyhow::Error> = ewc(|| Ok(ModuleArgs::F2(otr(args.get(0))?.try_into()?)));
            if let Ok(rt) = r {
                return Ok(rt);
            }

            let r: Result<ModuleArgs, anyhow::Error> = ewc(|| Ok(ModuleArgs::F3(otr(args.get(0))?.try_into()?)));
            if let Ok(rt) = r {
                return Ok(rt);
            }

            let r: Result<ModuleArgs, anyhow::Error> = ewc(|| Ok(ModuleArgs::F4(otr(args.get(0))?.try_into()?)));
            if let Ok(rt) = r {
                return Ok(rt);
            }

            Err(Error::new(proc_macro2::Span::call_site(), "Invalid args"))
        }
    }

    fn otr<T>(opt: Option<T>) -> Result<T, Error> {
        match opt {
            Some(val) => Ok(val),
            None => Err(Error::new(proc_macro2::Span::call_site(), "Invalid args")),
        }
    }

    fn ewc<F, T, E>(callback: F) -> Result<T, E>
    where
        F: FnOnce() -> Result<T, E>,
    {
        // 调用闭包并返回结果
        callback()
    }

    // #[args_object]
    // pub struct ModuleSubObj {
    //     pub imports: def::Array<def::Ident>,
    //     pub interceptors: def::Array<def::Ident>,
    //     pub controllers: def::Array<def::Ident>,
    //     pub services: def::Array<def::Ident>,
    //     pub exports: def::Array<def::Ident>,
    // }

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
}
