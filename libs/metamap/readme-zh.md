<div align="center">
  <h1>📦 Metamap</h1>
  <p><img src="https://github.com/nidrs/nidrs/blob/main/libs/metamap/logo.jpeg?raw=true" width="50%" /></p>
  <p>
    <img src="https://img.shields.io/crates/v/metamap?style=for-the-badge" />
  </p>
  <p>
    <a href="https://github.com/nidrs/nidrs/tree/main/libs/metamap">项目源码</a>
      ·
    <a href="https://github.com/nidrs/nidrs/blob/main/libs/metamap/readme-zh.md">中文文档</a>
  </p>
</div>

## 概述

Metamap 是一个用于存储任意类型的特殊 `map` 数据结构，旨在为不确定类型的存储场景提供灵活解决方案。它能够方便地存储和读取各种 Rust 原生类型以及自定义结构体或枚举类型，在高灵活性和泛化能力上表现出色。

## 安装

要将 Metamap 集成到你的项目中，请执行以下命令：

```shell
cargo add metamap
```

或手动在 `Cargo.toml` 文件中添加以下依赖项：

```toml
[dependencies]
metamap = "*"
```

## 使用说明

### 基础 KV 存取方式

你可以通过简单的键值对（KV）方式存取不同类型的数据，Metamap 支持整数、字符串、浮点数、向量等常见数据类型。

```rust
let mut meta = InnerMeta::new();
meta.set("a", 1);
meta.set("b", "2");
meta.set("c", 3.0);
meta.set("d", "4".to_string());
meta.set("e", vec![1, 2, 3]);
meta.set("f", vec!["1", "2", "3"]);
meta.set("g", vec![1.0, 2.0, 3.0]);
meta.set("h", vec!["1".to_string(), "2".to_string(), "3".to_string()]);
meta.set("i", vec![vec![1, 2], vec![3, 4]]);
meta.set("j", vec![vec!["1", "2"], vec!["3", "4"]]);
meta.set("k", vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
meta.set("l", vec![vec!["1".to_string(), "2".to_string()], vec!["3".to_string(), "4".to_string()]]);

assert_eq!(*meta.get::<i32>("a").unwrap(), 1);
assert_eq!(*meta.get::<&str>("b").unwrap(), "2");
assert_eq!(*meta.get::<f64>("c").unwrap(), 3.0);
assert_eq!(*meta.get::<String>("d").unwrap(), "4".to_string());
assert_eq!(*meta.get::<Vec<i32>>("e").unwrap(), vec![1, 2, 3]);
assert_eq!(*meta.get::<Vec<&str>>("f").unwrap(), vec!["1", "2", "3"]);
assert_eq!(*meta.get::<Vec<f64>>("g").unwrap(), vec![1.0, 2.0, 3.0]);
assert_eq!(*meta.get::<Vec<String>>("h").unwrap(), vec!["1".to_string(), "2".to_string(), "3".to_string()]);
assert_eq!(*meta.get::<Vec<Vec<i32>>>("i").unwrap(), vec![vec![1, 2], vec![3, 4]]);
assert_eq!(*meta.get::<Vec<Vec<&str>>>("j").unwrap(), vec![vec!["1", "2"], vec!["3", "4"]]);
```

### 存取复杂自定义类型

Metamap 不仅限于基础类型的存取，也支持复杂自定义类型如结构体和枚举。以下是存储和读取自定义类型的示例：

```rust
#[derive(Debug, PartialEq, Eq)]
enum TestEnum {
    A,
    B,
}

#[derive(Debug, PartialEq, Eq)]
struct TestData {
    pub name: String,
}

#[derive(Debug, PartialEq, Eq)]
struct TupleData(i32, String);

let mut meta = InnerMeta::new();
meta.set_data(TestEnum::A);
meta.set_data(TestData { name: "test".to_string() });
meta.set_data(TupleData(1, "tuple".to_string()));

assert_eq!(*meta.get_data::<TestEnum>().unwrap(), TestEnum::A);
assert_ne!(*meta.get_data::<TestEnum>().unwrap(), TestEnum::B);
assert_eq!(*meta.get_data::<TestData>().unwrap(), TestData { name: "test".to_string() });
assert_eq!(*meta.get_data::<TupleData>().unwrap(), TupleData(1, "tuple".to_string()));

assert_eq!(meta.take_data::<TestData>().unwrap(), TestData { name: "test".to_string() });
assert_eq!(meta.take_data::<TestEnum>().unwrap(), TestEnum::A);
assert_eq!(meta.take_data::<TupleData>().unwrap(), TupleData(1, "tuple".to_string()));

assert!(meta.get_data::<TestData>().is_none());
assert!(meta.get_data::<TestEnum>().is_none());
assert!(meta.get_data::<TupleData>().is_none());
```

### 更多示例

有关更多示例和高级用法，请查看 [GitHub 示例文件](https://github.com/nidrs/nidrs/blob/main/libs/metamap/src/lib.rs)。

## 关于 Metamap

Metamap 适用于那些需要灵活类型支持的项目，特别是在处理动态数据时表现出色。其设计初衷是为了减少不同类型存储和取回数据的复杂性。

- **许可协议**: 本项目使用 MIT 开源许可协议
- **更新日志**: 请参阅 [更新日志](https://github.com/nidrs/nidrs/blob/main/libs/metamap/CHANGELOG.md) 了解最新变化
