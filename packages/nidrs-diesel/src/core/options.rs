use nidrs_macro::injectable;

use crate::pool_manager::SqlitePoolManager;

#[injectable()]
#[derive(Default)]
pub struct DieselOptions {
    pub driver: ConnectionDriver,
}

#[derive(Default)]
pub enum ConnectionDriver {
    Sqlite(SqlitePoolManager),
    #[default]
    None,
}
