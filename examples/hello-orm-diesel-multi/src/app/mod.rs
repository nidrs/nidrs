use nidrs_macro::module;

pub mod controller;
pub mod dto;
pub mod exception;
pub mod service;

use crate::modules::user::UserModule;
use controller::AppController;
use service::AppService;

use nidrs_diesel::mysql::MysqlPoolManager;
use nidrs_diesel::sqlite::SqlitePoolManager;
use nidrs_diesel::DieselModule as DieselModule1;
use nidrs_diesel::DieselModule as DieselModule2;
use nidrs_diesel::DieselOptions;

#[module({
    imports: [
        DieselModule1::for_root(DieselOptions::new(MysqlPoolManager::new("mysql://root:12345678@127.0.0.1/hello-diesel")).with_name("Mysql")),
        DieselModule2::for_root(DieselOptions::new(SqlitePoolManager::new("file:db.sqlite3")).with_name("Sqlite")),
        UserModule,
    ],
    controllers: [AppController],
    services: [AppService],
    exports: [AppService],
})]
pub struct AppModule;
