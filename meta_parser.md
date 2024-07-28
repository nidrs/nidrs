# Meta 解析原理

## 编译模式 Meta 的形式

### `#[meta(key = value)]`

- key
  - 类型: &str
- value
  - 类型
    - 基础类型: number, string, boolean
    - 表达式类型：可以是复杂的表达式但是在编译环境下无法解析使用

### `#[meta(metadata(value))]`

- 支持编译模式: true

- metadata
  - 类型：一元组结构体
- value
  - 类型
    - 基础类型: number, string, boolean
    - 变量类型：就是简单的表达式，可以在编译环境下解析使用
      - 枚举: `nidrs::datasets::Global::Enabled`
        - 编译模式获取：meta::get_data<nidrs_extern::datasets::Global>()
        - 运行模式获取：同上
      - 一元组结构体: `nidrs::datasets::Global(基础类型)`
        - 编译模式获取：meta::get_data<nidrs_extern::datasets::Global>()
        - 运行模式获取：同上

### `#[meta(enum::value)]`

- enum::value
  - nidrs 内部变量在编译环境下获取
    - meta::get_data<nidrs_extern::datasets::Global>()
  - 其他不支持
