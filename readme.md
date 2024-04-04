<div align="center">
  <p><img src="readme.assets/log2.jpg" /></p>
  <p>
    <img alt="Discord" src="https://img.shields.io/discord/1223548737075281952?style=for-the-badge" />
    <img src="https://img.shields.io/github/last-commit/nidrs/nidrs?style=for-the-badge" />
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
- [ ] 请求响应拦截器 v0.0.4
- [ ] 请求参数校验
- [ ] 基于请求参数校验的 Mock 服务
- [ ] 统一返回类型
- [ ] 错误封装和处理
- [ ] 自动 OpenAPI
- [ ] 模块测试
- [ ] CLI 命令
- [ ] 完整的文档和例子

## Example

### example/src/app/controller.rs

```rs
use std::{collections::HashMap, sync::Arc};

use axum::{extract::{Query, State}, Json};
use nidrs::{Inject, StateCtx};
use nidrs_macro::{controller, get, post};

use super::service::AppService;

#[controller("/app")]
#[derive(Debug, Default)]
pub struct AppController {
    app_service: Inject<AppService>,
}

impl AppController {
    #[get("/hello")]
    pub async fn get_hello_world(&self, State(state): State<StateCtx>, Query(q): Query<HashMap<String, String>>) -> String {
        println!("Query {:?}", q);
        self.app_service.get_hello_world()
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
        self.user_service.get_hello_world()
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

use controller::AppController;
use service::AppService;
use crate::user::UserModule;
use crate::conf::ConfModule;
use crate::conf::ConfOptions;

#[module({
    imports = [
        ConfModule::for_root(ConfOptions{
            log_level: "info".to_string(),
        }),
        UserModule,
    ];
    controllers = [AppController];
    services = [AppService];
})]
#[derive(Clone, Debug, Default)]
pub struct AppModule;

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
[nidrs] Registering controller AppController.
[nidrs] Registering router 'GET /app/hello'.
[nidrs] Registering router 'POST /app/hello'.
[nidrs] Registering service AppService.
[nidrs] Registering dyn service ConfOptions.
[nidrs] Registering module ConfModule.
[nidrs] Registering service ConfService.
[nidrs] Injecting ConfService.
[nidrs] Triggering event on_module_init for ConfService.
ConfService initialized with log_level: ConfOptions { log_level: "info" }
[nidrs] Registering module UserModule.
[nidrs] Registering controller UserController.
[nidrs] Registering router 'GET /user/hello'.
[nidrs] Registering service UserService.
[nidrs] Injecting UserService.
[nidrs] Injecting UserController.
[nidrs] Injecting AppService.
[nidrs] Injecting AppController.
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

## About

整个框架目前处于早期阶段，0.x.x 都处于测试版本，正式稳定版本从 1.0 开始，不过如果你只是单纯的想找一个 axum 类的高层框架，而不需要后面的功能也可以尝试一下
最后如果有感兴趣的同学想要贡献和开发也可以加入下面的 Discord 一起来为 rust 世界添砖加瓦。

[欢迎加入 Discord](https://discord.gg/gwqKpxvUxU) ，微信群一起讨论交流

<img src="./readme.assets/image.png" alt="微信群" style="zoom: 25%;" />
