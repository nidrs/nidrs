use nidrs_macro::module;

pub mod controller;
pub mod dto;
pub mod exception;
pub mod service;

use crate::modules::user::UserModule;
use controller::AppController;
use service::AppService;

use nidrs_diesel::sqlite::SqlitePoolManager;
use nidrs_diesel::DieselModule;
use nidrs_diesel::DieselOptions;

#[module({
    imports: [
        DieselModule::for_root(DieselOptions{
            driver: SqlitePoolManager::new("file:db.sqlite3"),
        }),
        UserModule,
    ],
    controllers: [AppController],
    services: [AppService],
    exports: [AppService],
})]
pub struct AppModule;
