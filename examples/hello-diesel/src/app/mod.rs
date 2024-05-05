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
use nidrs_diesel::SqlitePoolManager;

#[default_uses(JsonInterceptor)]
#[module({
    imports: [
        DieselModule::for_root(DieselOptions{
            driver: SqlitePoolManager::new("file:db.sqlite3"),
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
