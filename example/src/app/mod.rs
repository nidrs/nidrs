#![allow(warnings, unused)]
use std::{any::Any, collections::HashMap, ptr::NonNull, rc::Rc, sync::{Arc, Mutex}};

use axum::Router;
use nestrs::{StateCtx, Inject, Module};
use nestrs_macro::module;

use crate::AppState;

pub mod controller;
pub mod service;

#[module(
  imports = [crate::user::UserModule],
  controllers = [controller::AppController],
  services = [service::AppService]
)]
pub struct AppModule;

impl nestrs::Module for AppModule {
    fn register(self, ctx: &nestrs::ModuleCtx) -> nestrs::DynamicModule {
      println!("Registering App Module");
      let base_router: Router<StateCtx> = axum::Router::new();

      let user_module = crate::user::UserModule::default();
      let user_module_dyn = user_module.register(ctx);


      ctx.services.lock().unwrap().insert("AppService".to_string(), Box::new(Inject::new(service::AppService::default())) as Box<dyn Any>);

      ctx.controllers.lock().unwrap().insert("AppController".to_string(), Box::new(Inject::new(controller::AppController::default())));
      
      // let base_router = base_router.merge(app_controller.register());

      // let mut routers = ctx.routers.lock().unwrap();
      // routers.push(base_router);
      
      nestrs::DynamicModule{
      }
    }
}

struct ModuleCtx{
  services: HashMap<String, Box<dyn Any>>
}