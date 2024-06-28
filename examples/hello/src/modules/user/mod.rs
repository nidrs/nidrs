use nidrs::macros::module;

pub mod controller;
pub mod dto;
pub mod interceptor;
pub mod service;

use crate::app::AppModule;
use controller::UserController;
use interceptor::UserInterceptor;
use service::UserService;

#[module({
  imports: [AppModule],
  controllers: [UserController],
  interceptors: [UserInterceptor],
  services: [UserService],
  exports: [UserService],
})]
pub struct UserModule;
