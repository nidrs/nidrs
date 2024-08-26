use nidrs_extern::anyhow;
use syn_args::def;
use syn_args::ArgsParse;
use syn_args_derive::ArgsParse;

#[derive(Debug, Clone, ArgsParse)]
pub enum ModuleArgs {
    F1(ModuleOptions),
}

#[derive(Debug, Clone, ArgsParse)]
pub struct ModuleOptions {
    pub imports: def::Option<def::Array<def::PathIdent>>,
    pub controllers: def::Option<def::Array<def::PathIdent>>,
    pub services: def::Option<def::Array<def::PathIdent>>,
    pub exports: def::Option<def::Array<def::PathIdent>>,
    pub interceptors: def::Option<def::Array<def::PathIdent>>,
}
