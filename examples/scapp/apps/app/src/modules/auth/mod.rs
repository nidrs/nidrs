use nidrs::macros::module;

pub mod controller;
pub mod dto;

use crate::modules::user::UserModule;
use controller::AuthController;

#[module({
  imports: [UserModule],
  controllers: [AuthController],
})]
pub struct AuthModule;
