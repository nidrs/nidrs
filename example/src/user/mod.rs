use nidrs_macro::module;

pub mod service;

use service::UserService;


#[module({
  services = [UserService];
  exports = [UserService];
})]
#[derive(Clone, Debug, Default)]
pub struct UserModule;