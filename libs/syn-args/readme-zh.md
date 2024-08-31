<div align="center">
  <h1>⚙️ SynArgs</h1>
  <p><img src="https://github.com/nidrs/nidrs/blob/main/libs/syn-args/logo.png?raw=true" width="50%" /></p>
  <p>
    <img src="https://img.shields.io/crates/v/syn-args?style=for-the-badge" />
  </p>
  <p>
    <a href="https://github.com/nidrs/nidrs/tree/main/libs/syn-args">仓库链接</a>
    ·
    <a href="https://github.com/nidrs/nidrs/blob/main/libs/syn-args/readme-zh.md">中文文档</a>
  </p>
</div>

## 你好

**SynArgs** 是一个强大且易用的字符串模式匹配和解析工具，能够将字符串解析为相应的数据结构。它广泛适用于多种参数解析需求，并且可以灵活扩展，适合复杂场景的参数解析。

## 特性

- **简单易用**：快速上手，轻松实现字符串到结构体的解析。
- **扩展性强**：支持扩展参数定义，适应多种解析场景。
- **支持多种基础类型**：内置对常见基础类型的支持，如数组、布尔值、整数等。
- **支持自定义类型**：灵活定义和解析自定义数据结构。
- **参数重置匹配**：通过枚举输出，实现复杂参数的重置和匹配。
- **多种参数解析模式**：
  - `F(P1, P2)`
  - `(P1, P2)`
  - `P1, P2`
- **基于 syn 开发**：利用 Rust 的 syn 库进行底层解析，稳定且高效。
- **宏辅助简化**：通过宏定义进一步简化参数解析流程。
  - `ArgsParse`
  - `syn_args::derive::declare`
  - `syn_args::derive::proc_attribute`

## 安装

在项目中添加依赖：

```shell
cargo add syn
cargo add syn-args
```

## 使用

### 字符串解析（底层用法）

以下是一些基本的字符串解析示例：

> 更多示例请参阅 [GitHub 示例文件](https://github.com/nidrs/nidrs/blob/main/libs/syn-args/examples/test.rs)

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

// 示例测试函数
fn test_formal_f3() {
    let res = ModuleArgs::parse("F(Hello)").unwrap();
    println!("{:?}", res);

    assert_eq!(res, ModuleArgs::F3(def::Expr("Hello".to_string())));
}

// 更多测试函数...
fn main() {
    test_formal_f3();
}
```

### TokenStream 解析（底层用法）

TokenStream 解析示例展示了如何使用 syn_args 解析带有复杂嵌套结构的 TokenStream：

> 查看完整示例：[GitHub 链接](https://github.com/nidrs/nidrs/blob/01bafd6c042e5585318df1c93df3cf1d0053277f/packages/nidrs-macro/src/args.rs)

类型定义:

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

使用示例:

```rs
let module_args = attr.meta.to_token_stream();
let module_options = syn::parse2::<syn_args::SynArgs>(module_args).unwrap().arguments::<syn_args::Arguments>().unwrap();
let module_options: ModuleOptions = module_options.try_into().unwrap();
```

### 宏用法（推荐高级用法）

宏用法大幅简化了参数解析流程，是推荐的高级用法：

> 查看完整示例：[GitHub 链接](https://github.com/nidrs/nidrs/blob/a7acea6a1be40da247299a53b1618a5c58752b15/packages/nidrs-macro/src/lib.rs)

使用示例：

```rust
#[default_uses(LogInterceptor, LogInterceptorB, LogInterceptorC)]
pub struct AppModule;
```

宏定义：

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

> **提示**：该方法仅适用于宏库开发环境下的 `[lib] proc-macro = true` 配置。

## 基础类型

- **def::Array**：数组类型。
- **def::Bool**：布尔类型。
- **def::Float**：浮点数类型。
- **def::Int**：整数类型。
- **def::Null**：空类型。
- **def::Object**：对象类型。
- **def::String**：字符串类型。
- **def::Expr**：表达式类型，支持 path 和 call 解析等。
- **def::Options**：可选参数类型。
- **def::Extends**：支持一个或多个参数，且必须是函数的最后一个参数。

具体类型定义请参考 [类型定义文档](https://github.com/nidrs/nidrs/blob/4c57b000adb6c36cbbc9d809f6915087ad468605/libs/syn-args/src/macro_args/def)。

## 关于

**许可证**：MIT

[查看更新日志](https://github.com/nidrs/nidrs/blob/main/libs/syn-args/CHANGELOG.md)
