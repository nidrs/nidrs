<div align="center">
  <h1>⚙️ SynArgs</h1>
  <p><img src="https://github.com/nidrs/nidrs/blob/main/libs/syn-args/logo.png?raw=true" width="50%" /></p>
  <p>
    <img src="https://img.shields.io/crates/v/syn-args?style=for-the-badge" />
  </p>
  <p>
    <a href="https://github.com/nidrs/nidrs/tree/main/libs/syn-args">Repo Link</a>
      ·
    <a href="https://github.com/nidrs/nidrs/blob/main/libs/syn-args/readme-zh.md">中文文档</a>
  </p>
</div>

## Hello

**SynArgs** is a powerful and easy-to-use string pattern matching and parsing tool that can parse strings into corresponding data structures. It is widely applicable to various parameter parsing needs and can be flexibly extended to suit complex parsing scenarios.

## Focus

- **Easy to Use**: Quickly get started and easily parse strings into structs.
- **Highly Extensible**: Supports extended parameter definitions to adapt to various parsing scenarios.
- **Supports Multiple Basic Types**: Built-in support for common basic types such as arrays, booleans, integers, etc.
- **Custom Type Support**: Flexibly define and parse custom data structures.
- **Parameter Reset Matching**: Achieve complex parameter resetting and matching through enum outputs.
- **Multiple Parameter Parsing Modes**:
  - `F(P1, P2)`
  - `(P1, P2)`
  - `P1, P2`
- **Built on syn**: Utilizes Rust's syn library for underlying parsing, ensuring stability and efficiency.
- **Macro Assisted Simplification**: Simplifies the parameter parsing process further through macro definitions.
  - `ArgsParse`
  - `syn_args::derive::declare`
  - `syn_args::derive::proc_attribute`

## Installation

Add the dependencies to your project:

```shell
cargo add syn
cargo add syn-args
```

## Features

- `syn-args = { features = ["loose_mode"] }`
  - Loose matching mode for parameters, where `def::Bool` and `def::Array` can be used without `def::Options` if they are optional parameters.

## Usage

### String Parsing (Basic Usage)

Below are some basic examples of string parsing:

> For more examples, see the [GitHub sample file](https://github.com/nidrs/nidrs/blob/main/libs/syn-args/examples/test.rs)

```rust
use syn::Error;
use syn_args::{def, derive::ArgsParse, ArgsParse, Formal};

#[derive(Debug, PartialEq, ArgsParse)]
pub enum ModuleArgs {
    F1(def::Int, def::Int),
    F2(def::Int),
    F3(def::Expr),
    F4(def::Array<def::Expr>),
    F5(ModuleSubObj),
    F6(def::Array<ModuleSubObj>),
}

#[derive(Debug, PartialEq, ArgsParse)]
pub struct ModuleSubObj {
    pub imports: def::Array<def::Expr>,
}

// Sample test function
fn test_formal_f3() {
    let res = ModuleArgs::parse("F(Hello)").unwrap();
    println!("{:?}", res);

    assert_eq!(res, ModuleArgs::F3(def::Expr("Hello".to_string())));
}

// More test functions...
fn main() {
    test_formal_f3();
}
```

### TokenStream Parsing (Basic Usage)

TokenStream parsing examples show how to use syn_args to parse TokenStreams with complex nested structures:

> View the full example: [GitHub Link](https://github.com/nidrs/nidrs/blob/01bafd6c042e5585318df1c93df3cf1d0053277f/packages/nidrs-macro/src/args.rs)

Type definition:

```rust
#[derive(Debug, Clone, ArgsParse)]
pub struct ModuleOptions {
    pub imports: def::Array<def::Expr>,
    pub controllers: def::Array<def::Expr>,
    pub services: def::Array<def::Expr>,
    pub exports: def::Array<def::Expr>,
    pub interceptors: def::Array<def::Expr>,
}
```

Usage example:

```rust
let module_args = attr.meta.to_token_stream();
let module_options = syn::parse2::<syn_args::SynArgs>(module_args).unwrap().arguments::<syn_args::Arguments>().unwrap();
let module_options: ModuleOptions = module_options.try_into().unwrap();
```

### Macro Usage (Recommended Advanced Usage)

Macro usage greatly simplifies the parameter parsing process and is the recommended advanced usage:

> View the full example: [GitHub Link](https://github.com/nidrs/nidrs/blob/a7acea6a1be40da247299a53b1618a5c58752b15/packages/nidrs-macro/src/lib.rs)

Usage example:

```rust
#[default_uses(LogInterceptor, LogInterceptorB, LogInterceptorC)]
pub struct AppModule;
```

Macro definition:

```rust
#[syn_args::derive::declare(def::Expr, def::Extends<def::Expr>)]
#[syn_args::derive::proc_attribute]
pub fn default_uses(args: Args, input: TokenStream) -> TokenStream {
    let args: Vec<def::Expr> = match args {
        Args::F1(first, other) => {
            // first => LogInterceptor
            // other => [LogInterceptorB, LogInterceptorC]
            let mut args = vec![first];
            args.append(&mut other.clone());
            args
        }
        _ => panic!("Invalid argument"),
    };
    let inter_names = args.iter().map(|arg| arg.to_path_name().unwrap()).collect::<Vec<String>>();

    DEFAULT_INTERS.lock().unwrap().append(&mut inter_names.clone());

    return input;
}
```

> **Note**: This method is only applicable in the macro library development environment with `[lib] proc-macro = true` configuration.

## Basic Types

- **def::Array**: Array type.
- **def::Bool**: Boolean type.
- **def::Float**: Float type.
- **def::Int**: Integer type.
- **def::Null**: Null type.
- **def::Object**: Object type.
- **def::String**: String type.
- **def::Expr**: Expression type, supports path and call parsing, etc.
- **def::Options**: Optional parameter type.
- **def::Extends**: Supports one or more parameters, and must be the last parameter of the function.

For specific type definitions, refer to the [Type Definition Documentation](https://github.com/nidrs/nidrs/blob/4c57b000adb6c36cbbc9d809f6915087ad468605/libs/syn-args/src/macro_args/def).

## About

**License**: MIT

[View the Changelog](https://github.com/nidrs/nidrs/blob/main/libs/syn-args/CHANGELOG.md)
