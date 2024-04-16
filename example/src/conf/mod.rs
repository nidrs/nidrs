pub mod service;
pub mod options;

use std::{any::Any, collections::HashMap, sync::Arc};

use nidrs::{DynamicModule, Service};
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
    DynamicModule{
      services: HashMap::from([
        ("ConfOptions", Box::new(Arc::new(options)) as Box<dyn Any + 'static>)
      ])
    }
  }
}