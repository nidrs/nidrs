use diesel::SqliteConnection;
use nidrs::{ImplMeta, Service};

use super::db_pool_manger::DbPoolManager;

#[derive(Default)]
pub struct DieselOptions<D: diesel::r2d2::R2D2Connection + 'static = SqliteConnection> {
    pub driver: DbPoolManager<D>,
}

// #[derive(Default)]
// pub struct DieselOptions<D: Into<ConnectionDriver> = ConnectionDriver> {
//     pub driver: D,
// }

impl Service for DieselOptions {}

impl ImplMeta for DieselOptions {
    fn __meta() -> nidrs::InnerMeta {
        let mut meta = nidrs::InnerMeta::new();
        meta.set("service_name", "DieselOptions");
        meta
    }
}

#[derive(Default)]
pub enum ConnectionDriver {
    #[cfg(feature = "sqlite")]
    Sqlite(crate::pool_manager::sqlite::SqlitePoolManager),

    #[cfg(feature = "mysql")]
    Mysql(crate::pool_manager::mysql::MysqlPoolManager),

    #[cfg(feature = "postgres")]
    Postgres(crate::pool_manager::postgres::PostgresPoolManager),

    #[default]
    None,
}
