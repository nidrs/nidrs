# TEMPLATE_NAME

a nidrs monorepo project.

## Start

```shell
cargo run -p app
```

## Project

- apps
  - app1
  - app2
  - ...
- libs
  - shared
  - macros
  - datasets
  - ...

### apps

Used for storing running services.

### libs

Used for storing tools, functions, custom macros, custom metadata, etc., required by the apps.

`shared` typically stores utility methods.

`macros` stores custom macros.

`datasets` stores custom metadata.

## About

nidrs
