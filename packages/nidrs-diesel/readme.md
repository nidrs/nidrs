# Nidrs & Diesel

This is a wrapper module for the Diesel ORM framework used in Nidrs to make it easier to use Diesel in Nidrs.

## Install

[Example](https://github.com/nidrs/nidrs/blob/80ba1fcc43d73473afa104e8dac132b2fdcfe1df/examples/hello-orm-diesel-mysql)

```toml
nidrs-diesel = { version = "*", features = ["sqlite"] }
diesel = { version = "2.1.6", default-features = false, features = ["sqlite", "chrono", "r2d2"] }
chrono = { version = "0.4.38", features = ["serde"] }
```

## Use

```rust
#[default_uses(JsonInterceptor)]
#[module({
    imports: [
        DieselModule::for_root(DieselOptions{
            driver: SqlitePoolManager::new("file:db.sqlite3"),
            // driver: MysqlPoolManager::new("mysql://root:12345678@localhost/hello-diesel"),
        }),
        UserModule,
    ],
    interceptors: [JsonInterceptor],
    controllers: [AppController],
    services: [AppService],
    exports: [AppService],
})]
#[derive(Clone, Debug, Default)]
pub struct AppModule;
```

## About

[nidrs](https://github.com/nidrs/nidrs)
