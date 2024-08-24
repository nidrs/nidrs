# SynArgs

This is a tool for string pattern matching and parsing to the corresponding data structure.

## Install

```shell
cargo add syn-args
```

## Use

```rs
use syn::Error;
use syn_args::{def, ArgsParse, Formal};
use syn_args_derive::ArgsParse;

#[derive(Debug, PartialEq, ArgsParse)]
pub enum ModuleArgs {
    F1(def::Int, def::Int),
    F2(def::Int),
    F3(def::Ident),
    F4(def::Array<def::Ident>),
    F5(ModuleSubObj),
    F6(def::Array<ModuleSubObj>),
}

#[derive(Debug, PartialEq, ArgsParse)]
pub struct ModuleSubObj {
    pub imports: def::Array<def::Ident>,
}

fn test_formal_f3() {
    let res = ModuleArgs::parse("F(Hello)").unwrap();
    println!("{:?}", res);

    assert_eq!(res, ModuleArgs::F3(def::Ident("Hello".to_string())));
}

fn test_formal_f4() {
    let res = ModuleArgs::parse("F([Ident1, Ident2])").unwrap();
    println!("{:?}", res);

    assert_eq!(res, ModuleArgs::F4(def::Array(vec![def::Ident("Ident1".to_string()), def::Ident("Ident2".to_string())])));
}
fn test_formal_f5() {
    let res = ModuleArgs::parse("F({ imports: [Ident1, Ident2] })").unwrap();
    println!("{:?}", res);

    assert_eq!(res, ModuleArgs::F5(ModuleSubObj { imports: def::Array(vec![def::Ident("Ident1".to_string()), def::Ident("Ident2".to_string())]) }));
}

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
fn main() {
    test_formal_f3();
    test_formal_f4();
    test_formal_f5();
    test_formal_f6();
}

```

## About

MIT
