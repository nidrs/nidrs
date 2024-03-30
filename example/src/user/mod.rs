use nidrs_macro::module;

pub mod service;
pub mod controller;

use service::UserService;
use controller::UserController;
use crate::app::AppModule;

#[module({
  imports = [AppModule];
  controllers = [UserController];
  services = [UserService];
  exports = [UserService];
})]
#[derive(Clone, Debug, Default)]
pub struct UserModule;