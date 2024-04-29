
use nidrs::{DynamicModule, Service};
use nidrs_macro::module;

pub mod service;
pub mod options;

pub use service::DieselService;
pub use options::DieselOptions;

#[module({
  services: [DieselService],
  exports: [DieselService],
})]
#[derive(Default)]
pub struct DieselModule;

impl DieselModule {
    pub fn for_root(opts: DieselOptions) -> DynamicModule {
        DynamicModule::new().provider(opts)
    }
}
