use nidrs_macro::module;

pub mod controller;
pub mod service;

use controller::AppController;
use service::AppService;
use crate::user::UserModule;
use crate::conf::ConfModule;
use crate::conf::ConfOptions;

#[module({
    imports = [
        ConfModule::for_root(ConfOptions{
            log_level: "info".to_string(),
        }),
        UserModule,
    ];
    controllers = [AppController];
    services = [AppService];
})]
#[derive(Clone, Debug, Default)]
pub struct AppModule;