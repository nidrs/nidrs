pub mod options;
pub mod service;

use nidrs::{meta, DynamicModule, Service};
use nidrs_macro::module;

pub use options::ConfOptions;
use service::ConfService;

#[meta(global)]
#[module({
  services: [ConfService],
  exports: [ConfService],
})]
#[derive(Clone, Debug, Default)]
pub struct ConfModule;

impl ConfModule {
    pub fn for_root(options: ConfOptions) -> DynamicModule {
        DynamicModule::new().service(options)
    }
}
