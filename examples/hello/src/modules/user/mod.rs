use nidrs::macros::module;

pub mod controller;
pub mod service;

use crate::app::AppModule;
use controller::UserController;
use service::UserService;

#[module({
  imports: [AppModule],
  controllers: [UserController],
  services: [UserService],
  exports: [UserService],
})]
pub struct UserModule;
