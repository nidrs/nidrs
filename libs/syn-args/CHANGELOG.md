# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## syn-args-v0.3.0 - 2024-08-31
#### Bug Fixes
- **(syn-args)** 兼容有名字的情况 - (92015b7) - *WumaCoder*
#### Documentation
- **(syn-args)** 补充文档和注释 - (44cb5f0) - *Lyda*
- **(syn-args)** 修改文档 - (de3be7c) - *Lyda*
- **(syn-args)** 优化文档 - (2bacf96) - *Lyda*
- **(syn-args)** 添加文档 - (40cf425) - *Lyda*
#### Features
- **(syn-args)** 支持可变的魔法类型 Extends - (44312da) - *Lyda*
- **(syn-args)** 宏兼容新的写法 - (c9714ba) - *WumaCoder*
- **(syn-args)** 给 Expr 添加一个实用的特性支持从里面提前 Path - (19fada1) - *Lyda*
- **(syn-args)** 支持更简单的使用 API - (b3fb129) - *WumaCoder*
- **(syn-args)** 添加新的辅助方法 - (402ec73) - *Lyda*
- **(syn-args)** 支持直接解析宏参数 - (c5d1d9c) - *WumaCoder*
- 支持才顶层参数中使用 Option - (3f11f69) - WumaCoder
#### Miscellaneous Chores
- **(syn-args)** 更新 syn-args 文档链接 - (b9bb425) - *Lyda*
- **(version)** syn-args-v0.2.1 - (f926439) - *Lyda*
- Revert "test: 零时提交" - (acfe461) - Lyda
#### Refactoring
- **(nidrs)** 使用 syn-args 重构细节 - (4c57b00) - *Lyda*
- **(syn-args)** 整理结构类型 - (f06dab3) - *WumaCoder*
- **(syn-args)** 重构 def::Array 的实现 - (658f99d) - *Lyda*
- 整理代码 - (2d1fbfd) - Lyda
- 去除无用库 - (f880473) - Lyda
- 添加 TryFrom<Value> for Arguments 的实现方法 - (ac8cc34) - Lyda
- 使用更通用的 Expr 类型去掉 PathIdent - (1291ef9) - Lyda
- 重新实现解析细节 - (e3a0fef) - WumaCoder
- 结构体支持解析一个参数的情况 - (b9dbc85) - WumaCoder
- add loose mode feature for array and bool parsing - (ee28ba7) - WumaCoder
- add default value for bool if not found in object - (27b1f09) - WumaCoder
- module args parse use syn-args - (ba8f5ac) - Lyda
- 重构 Object 的实现 - (3c6ecaa) - WumaCoder
- 重新实现 def::Ident - (62efae8) - Lyda
#### Style
- format - (58b88ed) - Lyda
- code format - (b384e37) - Lyda
#### Tests
- 零时提交 - (730c2ed) - WumaCoder

- - -

## syn-args-v0.2.1 - 2024-08-25
#### Bug Fixes
- **(syn-args)** 版本修复 - (f02d543) - *Lyda*

- - -

## syn-args-v0.2.0 - 2024-08-25
#### Documentation
- update doc - (cdfd0b5) - Lyda
#### Features
- **(syn-args)** 支持 def::Option 类型 - (04883be) - *Lyda*
#### Miscellaneous Chores
- **(syn-args)** 后续测试补充 - (159788c) - *Lyda*
#### Style
- code - (79b8690) - Lyda

- - -

## syn-args-v0.1.0 - 2024-08-24
#### Features
- 添加 syn-args-derive 和 syn-args 库的文档 - (bd078ee) - Lyda
- 完成 syn-args 库开发 - (a000497) - Lyda
- 为默认的类型实现 Deref - (7324a52) - Lyda
- 完成 parse 解析 - (50b41ad) - Lyda
- 添加 ModuleArgs 解析方法和测试 - (892c43f) - WumaCoder
- 创建 syn-args 库 - (ff1d663) - WumaCoder
- 实现宏展开 - (90f445d) - WumaCoder
#### Miscellaneous Chores
- **(version)** syn-args-v0.1.0 - (2f2e699) - *Lyda*
- **(version)** syn-args-v0.1.0 - (ba3204a) - *Lyda*
- **(version)** syn-args-v0.1.0 - (532233d) - *Lyda*
- **(version)** syn-args-v0.1.0 - (975e029) - *Lyda*
- Update syn-args-derive library to use workspace - (09833f2) - Lyda
- Update syn-args library category to "development-tools" - (2ca6516) - Lyda
- Update syn-args and syn-args-derive libraries to v0.1.0 - (c54827d) - Lyda
#### Refactoring
- Update syn-args library to latest version and add syn-args-derive documentation - (3f887ab) - Lyda
- Remove unused import in traits.rs - (c820c56) - Lyda
- 优化 API 和实现 - (8b892b6) - Lyda
- Update syn-args library to version 0.0.1 and syn-args-derive to version 0.0.1 - (58ed8e4) - WumaCoder
- 修改名称 - (bc2dde7) - WumaCoder
- Update TryFrom implementation for Object<ModuleSubObj> in syn-args library - (c04c8cc) - WumaCoder
- 支持递归处理 - (0f14437) - WumaCoder
- 修复项目名称 - (8f94b71) - WumaCoder

- - -

## syn-args-v0.1.0 - 2024-08-24
#### Features
- 添加 syn-args-derive 和 syn-args 库的文档 - (bd078ee) - Lyda
- 完成 syn-args 库开发 - (a000497) - Lyda
- 为默认的类型实现 Deref - (7324a52) - Lyda
- 完成 parse 解析 - (50b41ad) - Lyda
- 添加 ModuleArgs 解析方法和测试 - (892c43f) - WumaCoder
- 创建 syn-args 库 - (ff1d663) - WumaCoder
- 实现宏展开 - (90f445d) - WumaCoder
#### Miscellaneous Chores
- **(version)** syn-args-v0.1.0 - (ba3204a) - *Lyda*
- **(version)** syn-args-v0.1.0 - (532233d) - *Lyda*
- **(version)** syn-args-v0.1.0 - (975e029) - *Lyda*
- Update syn-args library category to "development-tools" - (2ca6516) - Lyda
- Update syn-args and syn-args-derive libraries to v0.1.0 - (c54827d) - Lyda
#### Refactoring
- Update syn-args library to latest version and add syn-args-derive documentation - (3f887ab) - Lyda
- Remove unused import in traits.rs - (c820c56) - Lyda
- 优化 API 和实现 - (8b892b6) - Lyda
- Update syn-args library to version 0.0.1 and syn-args-derive to version 0.0.1 - (58ed8e4) - WumaCoder
- 修改名称 - (bc2dde7) - WumaCoder
- Update TryFrom implementation for Object<ModuleSubObj> in syn-args library - (c04c8cc) - WumaCoder
- 支持递归处理 - (0f14437) - WumaCoder
- 修复项目名称 - (8f94b71) - WumaCoder

- - -

## syn-args-v0.1.0 - 2024-08-24
#### Features
- 添加 syn-args-derive 和 syn-args 库的文档 - (bd078ee) - Lyda
- 完成 syn-args 库开发 - (a000497) - Lyda
- 为默认的类型实现 Deref - (7324a52) - Lyda
- 完成 parse 解析 - (50b41ad) - Lyda
- 添加 ModuleArgs 解析方法和测试 - (892c43f) - WumaCoder
- 创建 syn-args 库 - (ff1d663) - WumaCoder
- 实现宏展开 - (90f445d) - WumaCoder
#### Miscellaneous Chores
- **(version)** syn-args-v0.1.0 - (532233d) - *Lyda*
- **(version)** syn-args-v0.1.0 - (975e029) - *Lyda*
- Update syn-args library category to "development-tools" - (2ca6516) - Lyda
- Update syn-args and syn-args-derive libraries to v0.1.0 - (c54827d) - Lyda
#### Refactoring
- Update syn-args library to latest version and add syn-args-derive documentation - (3f887ab) - Lyda
- Remove unused import in traits.rs - (c820c56) - Lyda
- 优化 API 和实现 - (8b892b6) - Lyda
- Update syn-args library to version 0.0.1 and syn-args-derive to version 0.0.1 - (58ed8e4) - WumaCoder
- 修改名称 - (bc2dde7) - WumaCoder
- Update TryFrom implementation for Object<ModuleSubObj> in syn-args library - (c04c8cc) - WumaCoder
- 支持递归处理 - (0f14437) - WumaCoder
- 修复项目名称 - (8f94b71) - WumaCoder

- - -

## syn-args-v0.1.0 - 2024-08-24
#### Features
- 添加 syn-args-derive 和 syn-args 库的文档 - (bd078ee) - Lyda
- 完成 syn-args 库开发 - (a000497) - Lyda
- 为默认的类型实现 Deref - (7324a52) - Lyda
- 完成 parse 解析 - (50b41ad) - Lyda
- 添加 ModuleArgs 解析方法和测试 - (892c43f) - WumaCoder
- 创建 syn-args 库 - (ff1d663) - WumaCoder
- 实现宏展开 - (90f445d) - WumaCoder
#### Miscellaneous Chores
- **(version)** syn-args-v0.1.0 - (975e029) - *Lyda*
- Update syn-args library category to "development-tools" - (2ca6516) - Lyda
- Update syn-args and syn-args-derive libraries to v0.1.0 - (c54827d) - Lyda
#### Refactoring
- Update syn-args library to latest version and add syn-args-derive documentation - (3f887ab) - Lyda
- Remove unused import in traits.rs - (c820c56) - Lyda
- 优化 API 和实现 - (8b892b6) - Lyda
- Update syn-args library to version 0.0.1 and syn-args-derive to version 0.0.1 - (58ed8e4) - WumaCoder
- 修改名称 - (bc2dde7) - WumaCoder
- Update TryFrom implementation for Object<ModuleSubObj> in syn-args library - (c04c8cc) - WumaCoder
- 支持递归处理 - (0f14437) - WumaCoder
- 修复项目名称 - (8f94b71) - WumaCoder

- - -

## syn-args-v0.1.0 - 2024-08-24
#### Features
- 添加 syn-args-derive 和 syn-args 库的文档 - (bd078ee) - Lyda
- 完成 syn-args 库开发 - (a000497) - Lyda
- 为默认的类型实现 Deref - (7324a52) - Lyda
- 完成 parse 解析 - (50b41ad) - Lyda
- 添加 ModuleArgs 解析方法和测试 - (892c43f) - WumaCoder
- 创建 syn-args 库 - (ff1d663) - WumaCoder
- 实现宏展开 - (90f445d) - WumaCoder
#### Refactoring
- Update syn-args library to latest version and add syn-args-derive documentation - (3f887ab) - Lyda
- Remove unused import in traits.rs - (c820c56) - Lyda
- 优化 API 和实现 - (8b892b6) - Lyda
- Update syn-args library to version 0.0.1 and syn-args-derive to version 0.0.1 - (58ed8e4) - WumaCoder
- 修改名称 - (bc2dde7) - WumaCoder
- Update TryFrom implementation for Object<ModuleSubObj> in syn-args library - (c04c8cc) - WumaCoder
- 支持递归处理 - (0f14437) - WumaCoder
- 修复项目名称 - (8f94b71) - WumaCoder

- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).