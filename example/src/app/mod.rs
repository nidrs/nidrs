use nidrs::default_uses;
use nidrs_macro::module;

pub mod controller;
pub mod dto;
pub mod exception;
pub mod service;

use crate::modules::conf::ConfModule;
use crate::modules::conf::ConfOptions;
use crate::modules::log::LogModule;
use crate::modules::user::UserModule;
use controller::AppController;
use service::AppService;

#[default_uses(LogInterceptor)]
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
