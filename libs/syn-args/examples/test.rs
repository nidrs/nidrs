use syn_args::{def, derive::ArgsParse, ArgsParse, Formal};

#[derive(Debug, PartialEq, ArgsParse)]
pub enum ModuleArgs {
    F1(def::Int, def::Int),
    F2(def::Int),
    F3(def::PathIdent),
    F4(def::Array<def::PathIdent>),
    F5(ModuleSubObj),
    F6(def::Array<ModuleSubObj>),
    F7(SubWrap),
}

#[derive(Debug, PartialEq, ArgsParse)]
pub struct ModuleSubObj {
    pub global: def::Option<def::Bool>,
    pub imports: def::Array<def::PathIdent>,
    pub sub: def::Option<Sub>,
}

#[derive(Debug, PartialEq, ArgsParse)]
pub struct Sub {
    pub value: def::Bool,
}

#[derive(Debug, PartialEq, ArgsParse)]
pub struct SubWrap {
    pub s1: Sub,
    pub s2: Sub,
}

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

fn test_formal_f2() {
    let f = Formal::new();

    let args = f.parse("F(1)").unwrap();
    // let args = f.parse("F(Hello)").unwrap();
    println!("{:?}", args);

    let res = ModuleArgs::try_from(&args).unwrap();
    println!("{:?}", res);

    assert_eq!(res, ModuleArgs::F2(def::Int(1)));
}

fn test_formal_f3() {
    let res = ModuleArgs::parse("F(Hello)").unwrap();
    println!("{:?}", res);

    assert_eq!(res, ModuleArgs::F3(def::PathIdent::from("Hello")));
}

fn test_formal_f4() {
    let res = ModuleArgs::parse("F([Ident1, Ident2])").unwrap();
    println!("{:?}", res);

    assert_eq!(res, ModuleArgs::F4(def::Array(vec![def::PathIdent::from("Ident1"), def::PathIdent::from("Ident2")])));
}

fn test_formal_f5() {
    let res = ModuleArgs::parse("F({ imports: [Ident1, Ident2] })").unwrap();
    println!("{:?}", res);

    assert_eq!(
        res,
        ModuleArgs::F5(ModuleSubObj {
            imports: def::Array(vec![def::PathIdent::from("Ident1"), def::PathIdent::from("Ident2")]),
            global: def::Option(None),
            sub: def::Option(None)
        })
    );
}

fn test_formal_f6() {
    let res = ModuleArgs::parse("F([{ imports: [Ident1, Ident2] }, { imports: [Ident3, Ident4] }])").unwrap();
    println!("{:?}", res);

    assert_eq!(
        res,
        ModuleArgs::F6(def::Array(vec![
            ModuleSubObj {
                imports: def::Array(vec![def::PathIdent::from("Ident1"), def::PathIdent::from("Ident2")]),
                global: def::Option(None),
                sub: def::Option(None)
            },
            ModuleSubObj {
                imports: def::Array(vec![def::PathIdent::from("Ident3"), def::PathIdent::from("Ident4")]),
                global: def::Option(None),
                sub: def::Option(None)
            }
        ]))
    );
}

fn test_formal_f6_2() {
    let res = ModuleArgs::parse("F([{ imports: [Ident1, Ident2], global: true }, { imports: [Ident3, Ident4] }])").unwrap();
    println!("{:?}", res);

    assert_eq!(
        res,
        ModuleArgs::F6(def::Array(vec![
            ModuleSubObj {
                imports: def::Array(vec![def::PathIdent::from("Ident1"), def::PathIdent::from("Ident2")]),
                global: def::Option(Some(Box::new(def::Bool(true)))),
                sub: def::Option(None)
            },
            ModuleSubObj {
                imports: def::Array(vec![def::PathIdent::from("Ident3"), def::PathIdent::from("Ident4")]),
                global: def::Option(None),
                sub: def::Option(None)
            }
        ]))
    );
}

fn test_formal_f6_3() {
    let res = ModuleArgs::parse("F([{ imports: [Ident1, Ident2], global: true, sub: { value: true } }, { imports: [Ident3, Ident4] }])").unwrap();
    println!("{:?}", res);

    assert_eq!(
        res,
        ModuleArgs::F6(def::Array(vec![
            ModuleSubObj {
                imports: def::Array(vec![def::PathIdent::from("Ident1"), def::PathIdent::from("Ident2")]),
                global: def::Option(Some(Box::new(def::Bool(true)))),
                sub: def::Option(Some(Box::new(Sub { value: def::Bool(true) })))
            },
            ModuleSubObj {
                imports: def::Array(vec![def::PathIdent::from("Ident3"), def::PathIdent::from("Ident4")]),
                global: def::Option(None),
                sub: def::Option(None)
            }
        ]))
    );
}

fn test_formal_f7() {
    let res = ModuleArgs::parse("F({ s1: { value: false }, s2: { value: true } })").unwrap();
    println!("{:?}", res);

    assert_eq!(res, ModuleArgs::F7(SubWrap { s1: Sub { value: def::Bool(false) }, s2: Sub { value: def::Bool(true) } }));
}

fn test_tokens_formal_f7() {
    let res = ModuleArgs::parse("F({ s1: { value: false }, s2: { value: true } })").unwrap();
    println!("{:?}", res);

    assert_eq!(res, ModuleArgs::F7(SubWrap { s1: Sub { value: def::Bool(false) }, s2: Sub { value: def::Bool(true) } }));
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
}
