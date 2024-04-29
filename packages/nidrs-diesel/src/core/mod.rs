use nidrs::{DynamicModule, Service};
use nidrs_macro::module;

pub mod options;
pub mod service;

pub use options::DieselOptions;
pub use service::DieselService;

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
