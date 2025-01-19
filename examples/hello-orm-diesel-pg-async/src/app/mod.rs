use nidrs_macro::module;

pub mod controller;
pub mod dto;
pub mod exception;
pub mod service;

use crate::modules::user::UserModule;
use controller::AppController;
use service::AppService;

use nidrs_diesel::postgres::PostgresPoolManager;
use nidrs_diesel::DieselModule;
use nidrs_diesel::DieselOptions;

#[module({
    imports: [
        DieselModule::for_root(DieselOptions::new(PostgresPoolManager::new("postgres://postgres:12345678@127.0.0.1:5432/hello-diesel"))),
        UserModule,
    ],
    controllers: [AppController],
    services: [AppService],
    exports: [AppService],
})]
pub struct AppModule;
