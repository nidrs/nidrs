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
    pub imports: def::Array<def::PathIdent>,
    pub controllers: def::Array<def::PathIdent>,
    pub services: def::Array<def::PathIdent>,
    pub exports: def::Array<def::PathIdent>,
    pub interceptors: def::Array<def::PathIdent>,
}
