pub mod options;
pub mod service;

use nidrs::{global, macros::module};
use nidrs::{meta, DynamicModule, Service};

pub use options::ConfOptions;
use service::ConfService;

#[global]
#[module({
  services: [ConfService],
  exports: [ConfService],
})]
pub struct ConfModule;

impl ConfModule {
    pub fn for_root(options: ConfOptions) -> DynamicModule {
        DynamicModule::new().service(options)
    }
}
