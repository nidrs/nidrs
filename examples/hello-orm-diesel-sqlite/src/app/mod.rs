use diesel::SqliteConnection;
use nidrs_macro::module;

pub mod controller;
pub mod dto;
pub mod exception;
pub mod service;

use crate::modules::user::UserModule;
use controller::AppController;
use service::AppService;

use nidrs_diesel::db_pool_manger::DbPoolManager;
use nidrs_diesel::DieselModule;
use nidrs_diesel::DieselOptions;
// use nidrs_diesel::MysqlPoolManager;

#[module({
    imports: [
        DieselModule::<SqliteConnection>::for_root(DieselOptions{
            driver: DbPoolManager::new("file:db.sqlite3"),
            // driver: SqlitePoolManager::new("file:db.sqlite3"),
            // driver: MysqlPoolManager::new("mysql://root:12345678@localhost/hello-diesel"),
        }),
        UserModule,
    ],
    controllers: [AppController],
    services: [AppService],
    exports: [AppService],
})]
pub struct AppModule;
