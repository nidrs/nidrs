use nidrs_macro::module;

pub mod controller;
pub mod dto;
pub mod exception;
pub mod service;

use crate::conf::ConfModule;
use crate::conf::ConfOptions;
use crate::log::LogModule;
use crate::user::UserModule;
use controller::AppController;
use service::AppService;

#[module({
    imports = [
        ConfModule::for_root(ConfOptions{
            log_level: "info".to_string(),
        }),
        LogModule,
        UserModule,
    ];
    controllers = [AppController];
    services = [AppService];
})]
#[derive(Clone, Debug, Default)]
pub struct AppModule;
