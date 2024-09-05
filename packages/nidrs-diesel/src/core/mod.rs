use nidrs::datasets::Global;
use nidrs::{meta, DynamicModule, Service};
use nidrs_macro::module;

pub mod db_pool_manger;
pub mod options;
pub mod pool_manager;
pub mod service;

pub use options::DieselOptions;
pub use pool_manager::PoolManager;
pub use service::DieselService;

// #[meta(Global(true))]
// #[module({
//   services: [DieselService],
//   exports: [DieselService],
// })]
// pub struct DieselModule;

// impl DieselModule {
//     pub fn for_root<D: Into<ConnectionDriver>>(opts: DieselOptions<D>) -> DynamicModule {
//         let d = DynamicModule::new();
//         match opts.driver.into() {
//             #[cfg(feature = "sqlite")]
//             options::ConnectionDriver::Sqlite(pool) => d.export(pool),
//             #[cfg(feature = "mysql")]
//             options::ConnectionDriver::Mysql(pool) => d.export(pool),
//             #[cfg(feature = "postgres")]
//             options::ConnectionDriver::Postgres(pool) => d.export(pool),
//             _ => d,
//         }
//     }

//     pub fn register() -> DieselModule {
//         DieselModule
//     }
// }
#[meta(Global(true))]
#[module({
  services: [DieselService],
  exports: [DieselService],
})]
pub struct DieselModule<D: diesel::r2d2::R2D2Connection + 'static>(Option<std::marker::PhantomData<D>>);

impl<D: diesel::r2d2::R2D2Connection + 'static> DieselModule<D> {
    pub fn for_root(opts: DieselOptions<D>) -> DynamicModule<DieselModule<D>> {
        let d = DynamicModule::new(DieselModule::<D>(None));
        d.export(opts.driver)
    }

    pub fn register() -> DieselModule<D> {
        DieselModule(Some(std::marker::PhantomData))
    }
}
