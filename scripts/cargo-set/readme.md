# cargo set

[REPO](https://github.com/nidrs/nidrs/tree/main/scripts/cargo-set)

This is a Cargo plugin for quickly setting up the contents of a Cargo.toml file.

## Install

```shell
cargo install cargo-set
```

## Use

./Cargo.toml

```toml
[package]
name = "cargo-set"
version = "0.1.0"
publish = true
```

```shell
cargo set package.name "test" 
cargo set package.version "\"0.2.0\"" 
cargo set package.publish false
```

> !: Numeric strings need to be wrapped in \"0.2.0\".
> Currently, only setting Int, Float, Bool, and String is supported.

Update ./Cargo.toml:

```toml
[package]
name = "test"
version = "0.2.0"
publish = false
```

## About

[nidrs](https://github.com/nidrs/nidrs)
