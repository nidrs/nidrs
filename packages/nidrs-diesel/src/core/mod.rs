use nidrs::metadata::Global;
use nidrs::{meta, DynamicModule, Service};
use nidrs_macro::module;

pub mod options;
pub mod pool_manager;
pub mod service;

pub use options::DieselOptions;
pub use pool_manager::PoolManager;
pub use service::DieselService;

use crate::ConnectionDriver;

#[meta(Global(true))]
#[module({
  services: [DieselService],
  exports: [DieselService],
})]
pub struct DieselModule;

impl DieselModule {
    pub fn for_root<D: Into<ConnectionDriver>>(opts: DieselOptions<D>) -> DynamicModule {
        let d = DynamicModule::new();
        match opts.driver.into() {
            #[cfg(feature = "sqlite")]
            options::ConnectionDriver::Sqlite(pool) => d.export(pool),
            #[cfg(feature = "mysql")]
            options::ConnectionDriver::Mysql(pool) => d.export(pool),
            #[cfg(feature = "postgres")]
            options::ConnectionDriver::Postgres(pool) => d.export(pool),
            _ => d,
        }
    }

    pub fn register() -> DieselModule {
        DieselModule
    }
}
