extern crate diesel;

pub mod core;
pub use core::*;

pub use core::options::ConnectionDriver;
pub use core::options::DieselOptions;
pub use core::pool_manager::PoolManager;
pub use core::service::DieselService;

#[cfg(feature = "mysql")]
pub use core::pool_manager::mysql::MysqlPoolManager;
#[cfg(feature = "sqlite")]
pub use core::pool_manager::sqlite::SqlitePoolManager;
