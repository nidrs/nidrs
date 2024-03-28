use std::{collections::HashMap, sync::Arc};

use nestrs::{Inject, ModuleCtx, Service, Controller};
use nestrs_macro::module;

pub mod service;

use service::UserService;


#[module({
  services = [UserService];
  exports = [UserService];
})]
#[derive(Clone, Debug, Default)]
pub struct UserModule;

// impl nestrs::Module for UserModule {
//     fn register(self, ctx: &ModuleCtx) -> nestrs::DynamicModule {
//       ctx.services.lock().unwrap().insert("user_service".to_string(), Box::new(Inject::new(service::UserService::default())) as Box<(dyn std::any::Any + 'static)>);
//       nestrs::DynamicModule{
//       }
//     }
// }