<div align="center">
  <p><img src="readme.assets/log2.jpg" /></p>
  <p>
    <img src="https://img.shields.io/discord/1223548737075281952?style=for-the-badge" />
    <img src="https://img.shields.io/crates/v/nidrs?style=for-the-badge" />
    <img src="https://img.shields.io/github/license/nidrs/nidrs?style=for-the-badge" />
  </p>
  <p>
    <a href="https://github.com/nidrs/nidrs/blob/main/readme_zh.md">中文文档</a>
  </p>
</div>

# Nidrs

Tributing the NestJS framework, Nidrs is a Rust-based enterprise-level modular development framework that draws inspiration from NestJS ideology while being developed and designed on Axum.

Nidrs provides a plug-and-play application architecture enabling developers and teams to effortlessly create highly testable, scalable, loosely coupled, and maintainable applications.

> NestJS is a framework for building efficient, scalable Node.js server-side applications. It uses progressive JavaScript, is built with and fully supports TypeScript (but still allows developers to write code in pure JavaScript), and combines elements of OOP (Object Oriented Programming), FP (Functional Programming), and FRP (Functional Reactive Programming).

## Focus

- [x] Modular encapsulation v0.0.1
  - [x] Static module registration v0.0.1
  - [x] Configurable module registration v0.0.2
- [x] Dependency auto-injection
  - [x] Service auto-injection v0.0.1
  - [x] Dynamic service injection v0.0.3
  - [x] Service scope (global) v0.0.1
  - [ ] Service scope (module)
  - [x] Service instance scope (singleton) v0.0.1
  - [ ] Service instance scope (request-level)
  - [ ] Service instance scope (injection-level)
- [x] Layered architecture
  - [x] Controller layer v0.0.1
  - [x] Service layer v0.0.1
  - [ ] Model layer
- [x] Module lifecycle hooks
  - [x] on_module_init v0.0.2
  - [ ] on_module_destroy
  - [ ] on_application_bootstrap
  - [ ] on_application_shutdown
- [x] Request-response interceptors v0.0.4
- [ ] Request parameter validation
- [ ] Mock service based on request parameter validation
- [x] Unified return type v0.0.4
- [x] Error encapsulation and handling v0.0.4
- [ ] Uniformly adding route prefixes
  - [ ] default_prefix
- [ ] Interface versioning
  - [ ] default_version
- [ ] Automatic OpenAPI
- [ ] Module testing
- [ ] CLI commands
- [ ] Comprehensive documentation and examples

## Example

### example/src/app/controller.rs

```rs
use std::{collections::HashMap, sync::Arc};

use axum::{extract::{Query, State}, http::{version, StatusCode}, Json};
use nidrs::{throw, version, Exception, Inject, StateCtx};
use nidrs_macro::{controller, get, meta, post, uses};

use crate::{shared::fn_test::fn_test, AppError, AppResult};

use super::{dto::{Status}, service::AppService};

// #[uses(LogInterceptor)]
#[version("v1")]
#[meta(role = "admin", auth = "true")]
#[meta(test = true)]
#[controller("/app")]
#[derive(Debug, Default)]
pub struct AppController {
    app_service: Inject<AppService>,
}

impl AppController {
    #[meta(arr = ["user"])]
    #[uses(LogInterceptor)]
    #[version("v2")]
    #[get("/hello")]
    pub async fn get_hello_world(&self, Query(q): Query<HashMap<String, String>>) -> AppResult<Status> {
        println!("Query {:?}", q);
        // fn_test()?;
        Ok(Status { db: "ok".to_string(), redis: "ok".to_string() })
    }

    #[uses(LogInterceptor)]
    #[get("/hello2")]
    pub async fn get_hello_world2(&self, Query(q): Query<HashMap<String, String>>) -> AppResult<String> {
        println!("Query {:?}", q);
        Ok(self.app_service.get_hello_world())
    }
    
    #[uses(LogInterceptor)]
    #[post("/hello")]
    pub async fn post_hello_world(&self, Query(q): Query<HashMap<String, String>>, Json(j): Json<serde_json::Value>) -> AppResult<String> {
        println!("Query {:?}", q);
        println!("Json {:?}", j);

        Ok("Hello, World2!".to_string())
    }
}

```

### example/src/app/service.rs

```rs
use nidrs::Inject;
use nidrs_macro::{injectable, on_module_init};
use crate::user::service::UserService;

#[injectable()]
#[derive(Clone, Debug, Default)]
pub struct AppService{
    user_service: Inject<UserService>
}

impl AppService {
    pub fn get_hello_world(&self) -> String {
        self.user_service.extract().get_hello_world()
    }

    pub fn get_hello_world2(&self) -> String {
        "Hello, nidrs2xx333!".to_string()
    }
}
```

### example/src/app/mod.rs

```rs
use nidrs_macro::module;

pub mod controller;
pub mod service;
pub mod dto;
pub mod exception;

use controller::AppController;
use service::AppService;
use crate::user::UserModule;
use crate::log::LogModule;
use crate::conf::ConfModule;
use crate::conf::ConfOptions;
use crate::log::interceptor::LogInterceptor;

#[module({
    imports = [
        ConfModule::for_root(ConfOptions{
            log_level: "info".to_string(),
        }),
        LogModule,
        UserModule,
    ];
    interceptors = [LogInterceptor];
    controllers = [AppController];
    services = [AppService];
})]
#[derive(Clone, Debug, Default)]
pub struct AppModule;

```

### example/src/main.rs

```rs
mod app;
mod conf;
mod user;
mod log;
mod shared;

pub use nidrs::AppResult;
pub use nidrs::AppError;

#[nidrs::main]
fn main() {
    let app = nidrs::NidrsFactory::create(app::AppModule);

    let app = app.default_prefix("/api/{version}");
    let app = app.default_version("v1");

    app.listen(3000);
}

```

Run Example:

```shell
git clone https://github.com/nidrs/nidrs.git
cd nidrs/example
cargo run
```

Launch Log:

```log
[nidrs] Registering module AppModule.
[nidrs] Registering interceptor LogInterceptor.
[nidrs] Registering controller AppController.
[nidrs] Registering router 'GET /api/v1/app/hello2'.
[nidrs] Registering router 'POST /api/v1/app/hello'.
[nidrs] Registering router 'GET /api/v2/app/hello'.
[nidrs] Registering service AppService.
[nidrs] Registering dyn service ConfOptions.
[nidrs] Registering module ConfModule.
[nidrs] Registering service ConfService.
[nidrs] Injecting ConfService.
[nidrs] Triggering event on_module_init for ConfService.
ConfService initialized with log_level: ConfOptions { log_level: "info" }
[nidrs] Registering module LogModule.
[nidrs] Registering service LogService.
[nidrs] Injecting LogService.
[nidrs] Registering module UserModule.
[nidrs] Registering controller UserController.
[nidrs] Registering router 'GET /api/v1/user/hello'.
[nidrs] Registering service UserService.
[nidrs] Injecting UserService.
[nidrs] Injecting UserController.
[nidrs] Injecting AppService.
[nidrs] Injecting AppController.
[nidrs] Injecting LogInterceptor.
[nidrs] Listening on 0.0.0.0:3000
```

## Sponsor

Open source is not easy, and sponsorship is also not easy. Both require courage. I believe that those who give love will receive love in return. May your good fortune start from this contribution~

[Go to Sponsor on [AfDian]](https://afdian.net/a/nidrs)

## About

The entire framework is currently in its early stages, with versions in the 0.x.x range being in testing. The stable version starts from 1.0. However, if you're simply looking for a high-level framework similar to Axum without needing the additional features mentioned later on, you can still give it a try.

Finally, if there are any interested individuals who wish to contribute and develop, they are welcome to join the Discord server below to contribute to the Rust community.

[Discord Server Link](https://discord.gg/gwqKpxvUxU)