use nidrs::module;

pub mod controller;
pub mod dto;
pub mod service;

use controller::UserController;
use service::UserService;

use crate::models::entities::user::UserEntity;

#[module({
  controllers: [UserController],
  services: [UserService, UserEntity],
  exports: [UserService],
})]
pub struct UserModule;
