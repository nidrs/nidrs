<div align="center">
  <p><img src="readme.assets/log2.jpg" /></p>
  <p>
    <img src="https://img.shields.io/discord/1223548737075281952?style=for-the-badge" />
    <img src="https://img.shields.io/crates/v/nidrs?style=for-the-badge" />
    <img src="https://img.shields.io/github/license/nidrs/nidrs?style=for-the-badge" />
</p>
</div>

# Nidrs

致敬 Nestjs 框架，Nidrs 是参考 Nestjs 思想的 Rust 企业级模块化开发框架，同时基于 Axum 进行开发和设计。

Nidrs 提供了一个即插即用的应用程序架构，使开发人员和团队能够轻松创建高度可测试、可扩展、松散耦合且易于维护的应用程序。

> Nestjs 是一个用于构建高效，可扩展的 Node.js 服务器端应用程序的框架。它使用渐进式 JavaScript，内置并完全支持 TypeScript（但仍然允许开发人员使用纯 JavaScript 编写代码）并结合了 OOP（面向对象编程），FP（函数式编程）和 FRP（函数式响应编程）的元素。

## Focus

- [x] 模块化封装 v0.0.1
  - [x] 静态模块注册 v0.0.1
  - [x] 可配置的模块注册 v0.0.2
- [x] 依赖自动注入
  - [x] service 自动注入 v0.0.1
  - [x] 动态 service 注入 v0.0.3
  - [x] service 作用域（全局）v0.0.1
  - [ ] service 作用域（模块）
  - [x] service 实例域（单例）v0.0.1
  - [ ] service 实例域（请求级）
  - [ ] service 实例域（注入级）
- [x] 分层架构
  - [x] Controller 层 v0.0.1
  - [x] Service 层 v0.0.1
  - [ ] Model 层
- [x] 模块生命周期钩子
  - [x] on_module_init v0.0.2
  - [ ] on_module_destroy
  - [ ] on_application_bootstrap
  - [ ] on_application_shutdown
- [x] 请求响应拦截器 v0.0.4
- [ ] 请求参数校验
- [ ] 基于请求参数校验的 Mock 服务
- [x] 统一返回类型 v0.0.4
- [x] 错误封装和处理 v0.0.4
- [ ] 统一添加路由前缀
  - [ ] default_prefix
- [ ] 接口版本控制
  - [ ] default_version
- [ ] 自动 OpenAPI
- [ ] 模块测试
- [ ] CLI 命令
- [ ] 完整的文档和例子

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

运行例子：

```shell
git clone https://github.com/nidrs/nidrs.git
cd nidrs/example
cargo run
```

运行日志：

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

## Design

整个框架的目标是提高模块复用和极致的开发便携。

框架整体是通过宏来自动收集依赖关系，并且生成服务注册、依赖注入、路由注册的相关代码。

框架初始化大概可以分为两大阶段：

    1. 模块注册
       1. 路由注册
       2. 服务注册
       3. 模块注册
    2. 依赖注入

框架在初始化的时候会创建一个核心的对象：

```rs
#[derive(Debug, Clone)]
pub struct ModuleCtx{
    pub services: Arc<Mutex<HashMap<String, Box<dyn Any>>>>,
    pub controllers: Arc<Mutex<HashMap<String, Box<dyn Any>>>>,
    pub routers: Arc<Mutex<Vec<axum::Router<StateCtx>>>>
}
```

而服务整个注册的行为就是给 `ctx.services.insert("AppService", AppService)` 插入创建的实例。

而依赖注入会将标记为 `Inject` 的属性找出来，从 `ctx.services` map 上获取实例化后的 service 赋值给对应的属性，所以目前所有的服务都是单例。

具体想看宏生成的代码可以执行 `cd nidrs/example && cargo expand` 即可。

## Sponsor

开源不易，赞助亦不易，都需要勇气，我相信爱出者爱返，愿你的好运从这份奉献开始运转～

[前往【爱发电】赞助](https://afdian.net/a/nidrs)

## About

整个框架目前处于早期阶段，0.x.x 都处于测试版本，正式稳定版本从 1.0 开始，不过如果你只是单纯的想找一个 axum 类的高层框架，而不需要后面的功能也可以尝试一下
最后如果有感兴趣的同学想要贡献和开发也可以加入下面的 Discord 一起来为 rust 世界添砖加瓦。

[欢迎加入 Discord](https://discord.gg/gwqKpxvUxU) ，微信群一起讨论交流

<p>
<img src="./readme.assets/image.png" alt="微信群" style="width: 200px" />
</p>