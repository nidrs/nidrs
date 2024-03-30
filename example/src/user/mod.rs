use nidrs_macro::module;

pub mod service;
pub mod controller;

use service::UserService;
use controller::UserController;

#[module({
  controllers = [UserController];
  services = [UserService];
  exports = [UserService];
})]
#[derive(Clone, Debug, Default)]
pub struct UserModule;