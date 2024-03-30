use nidrs_macro::module;

pub mod controller;
pub mod service;

use controller::AppController;
use service::AppService;
use crate::user::UserModule;

#[module({
    imports = [UserModule];
    controllers = [AppController];
    services = [AppService];
})]
pub struct AppModule;
