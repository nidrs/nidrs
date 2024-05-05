use nidrs::{ImplMeta, Service};

use crate::pool_manager::SqlitePoolManager;

#[derive(Default)]
pub struct DieselOptions<D: Into<ConnectionDriver> = SqlitePoolManager> {
    pub driver: D,
}

impl Service for DieselOptions {}

impl ImplMeta for DieselOptions {
    fn __meta() -> nidrs::Meta {
        let mut meta = nidrs::Meta::new();
        meta.set("service_name", "DieselOptions");
        meta
    }
}

#[derive(Default)]
pub enum ConnectionDriver {
    Sqlite(SqlitePoolManager),
    #[default]
    None,
}
