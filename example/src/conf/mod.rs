pub mod service;
pub mod options;

use std::{any::Any, collections::HashMap, sync::Arc};

use nidrs::{provider, DynamicModule, Service};
use nidrs_macro::module;

use service::ConfService;
pub use options::ConfOptions;

#[module({
  services = [ConfService];
  exports = [ConfService];
})]
#[derive(Clone, Debug, Default)]
pub struct ConfModule;

impl ConfModule {
  pub fn for_root(options: ConfOptions) -> DynamicModule {
    DynamicModule::new()
      .provider(options)
  }
}