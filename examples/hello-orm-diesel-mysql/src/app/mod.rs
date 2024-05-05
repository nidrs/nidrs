use nidrs::default_uses;
use nidrs_macro::module;

pub mod controller;
pub mod dto;
pub mod exception;
pub mod service;

use crate::{interceptors::json_interceptor::JsonInterceptor, modules::user::UserModule};
use controller::AppController;
use service::AppService;

use nidrs_diesel::DieselModule;
use nidrs_diesel::DieselOptions;
// use nidrs_diesel::SqlitePoolManager;
use nidrs_diesel::MysqlPoolManager;

#[default_uses(JsonInterceptor)]
#[module({
    imports: [
        DieselModule::for_root(DieselOptions{
            // driver: SqlitePoolManager::new("file:db.sqlite3"),
            driver: MysqlPoolManager::new("mysql://root:12345678@127.0.0.1/hello-diesel"),
        }),
        UserModule,
    ],
    interceptors: [JsonInterceptor],
    controllers: [AppController],
    services: [AppService],
    exports: [AppService],
})]
#[derive(Clone, Debug, Default)]
pub struct AppModule;
