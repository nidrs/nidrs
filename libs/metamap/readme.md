<div align="center">
  <h1>📦 Metamap</h1>
  <p><img src="https://github.com/nidrs/nidrs/blob/main/libs/metamap/logo.jpeg?raw=true" width="50%" /></p>
  <p>
    <img src="https://img.shields.io/crates/v/metamap?style=for-the-badge" />
  </p>
  <p>
    <a href="https://github.com/nidrs/nidrs/tree/main/libs/metamap">Source Code</a>
      ·
    <a href="https://github.com/nidrs/nidrs/blob/main/libs/metamap/readme-zh.md">中文文档</a>
  </p>
</div>

## Overview

Metamap is a special `map` data structure for storing arbitrary types, designed to provide a flexible solution for scenarios where the type is uncertain. It allows you to easily store and retrieve various native Rust types as well as custom structs or enums, showcasing excellent flexibility and generalization capabilities.

## Installation

To integrate Metamap into your project, run the following command:

```shell
cargo add metamap
```

Alternatively, manually add the following dependency to your `Cargo.toml` file:

```toml
[dependencies]
metamap = "*"
```

## Usage

### Basic Key-Value (KV) Access

You can store and retrieve data of different types using a simple key-value (KV) approach. Metamap supports common data types such as integers, strings, floating-point numbers, vectors, and more.

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

### Storing and Accessing Complex Custom Types

Metamap is not limited to basic types; it also supports complex custom types such as structs and enums. Here is an example of storing and retrieving custom types:

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

### More Examples

For more examples and advanced usage, check out the [GitHub example file](https://github.com/nidrs/nidrs/blob/main/libs/metamap/src/lib.rs).

## About Metamap

Metamap is suitable for projects that require flexible type support, especially when dealing with dynamic data. It is designed to reduce the complexity of storing and retrieving data of different types.

- **License**: This project is licensed under the MIT License
- **Changelog**: See the [Changelog](https://github.com/nidrs/nidrs/blob/main/libs/metamap/CHANGELOG.md) for the latest updates.
