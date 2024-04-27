use nidrs_macro::module;

pub mod controller;
pub mod service;

use crate::app::AppModule;
use controller::UserController;
use service::UserService;

#[module({
  imports: [AppModule],
  controllers: [UserController],
  services: [UserService],
  export: [UserService],
})]
#[derive(Clone, Debug, Default)]
pub struct UserModule;
