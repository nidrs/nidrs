# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## nidrs-macro-v0.2.2 - 2025-01-06
#### Bug Fixes
- **(nidrs)** 不使用 root import 方式导入拦截器 - (20b6657) - *Lyda*
- remove disable_auto_json - (9fbb28e) - Lyda
#### Refactoring
- **(nidrs)** simplify error handling in interceptor response - (5df6daf) - *Lyda*
- format code - (54773d6) - Lyda

- - -

## nidrs-macro-v0.2.1 - 2024-11-15
#### Bug Fixes
- **(nidrs)** 修复空 exports 选项引发的错误 - (25b26d6) - *WumaCoder*
- **(nidrs-macro)** adapt impl meta - (04e316e) - *WumaCoder*
- **(nidrs-macro)** 修复对文件系统的依赖 - (57f8ba7) - *WumaCoder*
#### Miscellaneous Chores
- 优化错误提示 - (3c92988) - WumaCoder
- 移除无用代码 - (1522045) - Lyda
- commit - (fa7c6c0) - WumaCoder

- - -

## nidrs-macro-v0.2.0 - 2024-09-25
#### Documentation
- update - (c989f4d) - Lyda
#### Features
- **(nidrs)** 封装 merge_derives - (9e50d30) - *WumaCoder*
#### Miscellaneous Chores
- **(version)** nidrs-macro-v0.1.0 - (a6e7c67) - *Lyda*
#### Refactoring
- **(nidrs-extern)** 迁移核心的方法 - (9666d39) - *WumaCoder*
#### Style
- code - (5be8aea) - Lyda

- - -

## nidrs-macro-v0.1.0 - 2024-09-24
#### Bug Fixes
- **(nidrs)** 修复框架 route_method 重复 - (95ed16f) - *WumaCoder*
- **(nidrs)** 修复重构后的一些错误 - (7b7ec6f) - *Lyda*
- **(nidrs)** uses macro bug - (5056320) - *Lyda*
-  meta 支持回调参数 - (048e476) - Lyda
- 支持自动识别返回类型 - (2723065) - WumaCoder
- 使用新的路由注册逻辑 - (132a4fa) - WumaCoder
- 修复 MetaData 的 value 解析失败的问题 - (b504060) - WumaCoder
#### Documentation
- update ifdian.net - (29a2255) - Lyda
#### Features
- **(nidrs)** 添加 disable_auto_json meta - (72ba90b) - *WumaCoder*
- **(nidrs)** 支持 openapi 的参数解析 - (48a18d6) - *Lyda*
- 创建 valid 包 - (8b8afb4) - WumaCoder
#### Miscellaneous Chores
- 使用 loose_mode - (c3df074) - WumaCoder
- Revert "test: 零时提交" - (acfe461) - Lyda
- 临时测试 - (d29ab9b) - WumaCoder
- 升级基础库 - (48fb4d6) - WumaCoder
- 临时提交 - (04f97b4) - WumaCoder
- 重构 module 解析流程 - (e713538) - WumaCoder
#### Refactoring
- **(nidrs)** optimize macro generation - (aea3ad0) - *WumaCoder*
- **(nidrs)** 修改命名 - (f10cb46) - *Lyda*
- **(nidrs)** 删除旧的 Meta 实现 - (c037438) - *Lyda*
- **(nidrs)** 简化抽象实现函数 - (b6b66cb) - *Lyda*
- **(nidrs)** 使用 syn-args 重构细节 - (4c57b00) - *Lyda*
- **(nidrs)** 使用通用的解析器去解析 - (01bafd6) - *Lyda*
- **(nidrs)** 使用 syn-args 重构 version 宏 - (e63d4a4) - *WumaCoder*
- **(nidrs)** 使用 syn-args 重构 controller - (ab4e73d) - *WumaCoder*
- **(nidrs-macro)** 使用 UFnStruct 替换 InterceptorArgs - (15bed29) - *Lyda*
- update localhost - (8b68402) - Lyda
- 优化对泛型的支持 - (0a5c5fa) - Lyda
- Remove unused code and update dependencies - (81e145e) - Lyda
- 移除多余的代码 - (a7acea6) - Lyda
- 删除无用的类型 - (b7e9217) - WumaCoder
- 删除无用的类型 - (84ac0e9) - Lyda
- 使用更通用的 Expr 类型去掉 PathIdent - (1291ef9) - Lyda
- 使用新的方式重构 module - (3b82fcc) - WumaCoder
- module args parse use syn-args - (ba8f5ac) - Lyda
- 使用 syn-args 重构 expand_controller_register - (e7d963e) - WumaCoder
- 尝试使用 syn-args 重构 nidrs - (eaf4d0f) - Lyda
- Add ModuleArgs and ModuleSubObj structs for macro arguments parsing - (7908346) - WumaCoder
- Add macro_args module and implement Formal struct for parsing macro arguments - (035ede0) - WumaCoder
- Merge utils.rs and lib.rs changes - (92afff9) - Lyda
- 文件改名 - (df37fa8) - WumaCoder
- 移除 CURRENT_SERVICE 代码 - (d5bcfb8) - WumaCoder
- 删除 CURRENT_CONTROLLER 相关代码 - (0d34ecf) - WumaCoder
- 简化 ROUTES - (33bee54) - WumaCoder
- 移除无用的代码 - (e3f511d) - WumaCoder
- 完成拦截器的重构实现 - (76c3b69) - Lyda
- 重构 meta 相关的细节实现 - (61f8251) - WumaCoder
- 使用新的路由注册逻辑 - (31432cd) - WumaCoder
- 初步重构路由注册 - (d9314ee) - Lyda
- get_stack_data - (07c6295) - WumaCoder
- 重构路由注册方法 - (e9a9745) - WumaCoder
- 优化 cmeta.rs 文件中的代码结构 - (e8c3138) - WumaCoder
- 优化 meta tokens 的 build 过程 - (ba7fdda) - WumaCoder
- 添加 app_parse 模块以解析应用程序的宏参数 - (840f835) - WumaCoder
- 优化 meta 的读取流程和堆栈模式 - (442b31e) - WumaCoder
- 优化 meta 的读取流程和堆栈模式 - (ee2f0ec) - WumaCoder
- 完成 meta 的读取流程 - (b5d1248) - WumaCoder
- 设计 meta 堆栈的模式 - (3d19509) - WumaCoder
- 确定了 meta 在框架中的重要性，并且规范了在编译环境下 meta 的读写规范 - (3a9deb4) - WumaCoder
- 优化细节 - (c61c52d) - WumaCoder
- 重新构想模块解析实现 - (6ae2b58) - WumaCoder
#### Tests
- 零时提交 - (730c2ed) - WumaCoder

- - -

## nidrs-macro-v0.0.11 - 2024-06-12
#### Refactoring
- **(nidrs-macro)** 修改 metadata 为 datasets - (65f486d) - *WumaCoder*

- - -

## nidrs-macro-v0.0.10 - 2024-06-11
#### Documentation
- update readme - (ed7424d) - WumaCoder
#### Features
- 添加 RouterFullPath metadata - (32743ef) - WumaCoder
- 支持 tower 中间件 - (e0a126e) - WumaCoder
#### Refactoring
- 使用 meta.set_data 替代  meta.set 来提高使用体验 - (c563c39) - WumaCoder
- 重构添加 MATE_STACK - (4b84b8d) - WumaCoder
- 重构 Default derive 到  __controller_derive - (ab8e2f0) - WumaCoder
- 修改 metadata 的方法名为 value - (e16df2f) - WumaCoder
- 重构系统默认的 metadata 的实现 - (e134808) - WumaCoder
- 完善 metadata 在整个框架中配置作用 - (d35cc7e) - WumaCoder
#### Style
- format - (4658e92) - WumaCoder

- - -

## nidrs-macro-v0.0.9 - 2024-06-03
#### Features
- controller and method macro support empty param - (3fd0457) - WumaCoder
- Add route macros for any, head, on, options, patch, and trace methods - (7b5240a) - WumaCoder
#### Refactoring
- 优化 meta 和宏导出的细节 - (0460927) - WumaCoder
- default impl Default - (9becd92) - WumaCoder
- Add disable_default_prefix macro attribute - (8c34186) - WumaCoder
- 重构代码结构 - (eb0826f) - WumaCoder

- - -

## nidrs-macro-v0.0.8 - 2024-05-05
#### Bug Fixes
- 修复注册拦截器的 BUG - (860127f) - WumaCoder
- 优化框架细节 - (60744b5) - WumaCoder
- 修复 global 模块注入问题 - (e6edef0) - WumaCoder
- 修复 module 宏逻辑错误 - (259a6ea) - WumaCoder
#### Features
- 支持在宏编译阶段读取基础 meta - (31d9918) - WumaCoder
#### Miscellaneous Chores
- **(version)** nidrs-v0.0.7 - (d19d685) - *WumaCoder*
- update toml - (7cbe44e) - WumaCoder
#### Refactoring
- 优化 global service 获取逻辑 - (d36c717) - WumaCoder
- meta macro - (bd7bdb9) - WumaCoder
- rename meta - (faeee3a) - WumaCoder

- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).