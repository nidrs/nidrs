use nidrs::datasets::Global;
use nidrs::{meta, DynamicModule, Service};
use nidrs_macro::module;

pub mod drivers;
pub mod options;
pub mod service;

pub use options::DieselOptions;
pub use service::DieselService;

use crate::ConnectionDriver;

#[meta(Global(true))]
#[module({
  services: [DieselService],
  exports: [DieselService],
})]
pub struct DieselModule;

impl DieselModule {
    pub fn for_root<D: Into<ConnectionDriver>>(opts: DieselOptions<D>) -> DynamicModule<Self> {
        let d = DynamicModule::new(DieselModule);
        #[cfg(not(feature = "async"))]
        match opts.driver.into() {
            #[cfg(feature = "sqlite")]
            drivers::driver::ConnectionDriver::Sqlite(pool) => d.export2(pool, opts.name),
            #[cfg(feature = "mysql")]
            drivers::driver::ConnectionDriver::Mysql(pool) => d.export2(pool, opts.name),
            #[cfg(feature = "postgres")]
            drivers::driver::ConnectionDriver::Postgres(pool) => d.export2(pool, opts.name),
            _ => d,
        }
        #[cfg(feature = "async")]
        match opts.driver.into() {
            #[cfg(feature = "sqlite_async")]
            drivers::driver::ConnectionDriver::Sqlite(pool) => d.export2(pool, opts.name),
            #[cfg(feature = "mysql_async")]
            drivers::driver::ConnectionDriver::Mysql(pool) => d.export2(pool, opts.name),
            #[cfg(feature = "postgres_async")]
            drivers::driver::ConnectionDriver::Postgres(pool) => d.export2(pool, opts.name),
            _ => d,
        }
    }

    pub fn register() -> DieselModule {
        DieselModule
    }
}
