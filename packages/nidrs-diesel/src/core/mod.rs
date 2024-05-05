use nidrs::{meta, DynamicModule, Service};
use nidrs_macro::module;

pub mod options;
pub mod pool_manager;
pub mod service;

pub use options::DieselOptions;
pub use pool_manager::PoolManager;
pub use service::DieselService;

#[meta(global)]
#[module({
  services: [DieselService],
  exports: [DieselService],
})]
#[derive(Default)]
pub struct DieselModule;

impl DieselModule {
    pub fn for_root(opts: DieselOptions) -> DynamicModule {
        let d = DynamicModule::new();
        if let options::ConnectionDriver::Sqlite(pool) = opts.driver.into() {
            return d.export(pool);
        }
        d
    }

    pub fn register() -> DieselModule {
        DieselModule
    }
}
