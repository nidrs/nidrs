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

- [x] 模块化封装
  - [x] 静态模块注册
  - [ ] 可配置的模块注册
  - [ ] 可动态模块注册
- [x] 依赖自动注入
  - [x] service 自动注入
  - [ ] 动态 service 注入
  - [ ] service 作用域（全局、引入模块）
  - [ ] service 实例域（单例、请求级、注入级）
- [x] 分层架构
  - [x] 控制层
  - [x] 服务层
- [ ] 模块生命周期钩子
  - [ ] on_module_init
- [ ] 请求响应拦截器
- [ ] 统一返回类型
- [ ] 自动 OpenAPI
- [ ] 模块测试
- [ ] CLI 命令
- [ ] 完整的文档和例子

## About

整个框架目前处于早期阶段，0.x.x 都处于测试版本，正式稳定版本从 1.0 开始，不过如果你只是单纯的想找一个 axum 类的高层框架，而不需要后面的功能也可以尝试一下
最后如果有感兴趣的同学想要贡献和开发也可以加入下面的 Discord 一起来为 rust 世界添砖加瓦。

[欢迎加入 Discord](https://discord.gg/gwqKpxvUxU)

MIT
