extern crate diesel;

pub mod core;
pub use core::*;

pub use core::options::ConnectionDriver;
pub use core::options::DieselOptions;
pub use core::pool_manager::PoolManager;
pub use core::service::DieselService;

pub use core::pool_manager::SqlitePoolManager;
