pub mod options;
pub mod service;

use std::{any::Any, collections::HashMap, sync::Arc};

use nidrs::{provider, DynamicModule, Service};
use nidrs_macro::module;

pub use options::ConfOptions;
use service::ConfService;

#[module({
  services = [ConfService];
  exports = [ConfService];
})]
#[derive(Clone, Debug, Default)]
pub struct ConfModule;

impl ConfModule {
    pub fn for_root(options: ConfOptions) -> DynamicModule {
        DynamicModule::new().provider(options)
    }
}
