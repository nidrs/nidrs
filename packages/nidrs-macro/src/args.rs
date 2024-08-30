use nidrs_extern::anyhow;
use syn_args::def;
use syn_args::ArgsParse;
use syn_args_derive::ArgsParse;

#[derive(Debug, Clone, ArgsParse)]
pub struct ModuleOptions {
    pub imports: def::Array<def::Expr>,
    pub controllers: def::Array<def::Expr>,
    pub services: def::Array<def::Expr>,
    pub exports: def::Array<def::Expr>,
    pub interceptors: def::Array<def::Expr>,
}

#[derive(Debug, Clone, ArgsParse)]
pub enum RouteArgs {
    F1(def::Option<def::String>),
}
