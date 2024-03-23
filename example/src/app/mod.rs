#![allow(warnings, unused)]
use std::{any::Any, collections::HashMap, rc::Rc, sync::Arc};

use axum::Router;
use nestrs::{Ctx, Module};
use nestrs_macro::module;

use crate::AppState;

pub mod controller;
pub mod service;

#[module(
  controllers = [controller::AppController],
  services = [service::AppService]
)]
pub struct AppModule;

impl nestrs::Module for AppModule {
    fn register(self) -> nestrs::DynamicModule {
      println!("Registering App Module");
      let mut ctx = ModuleCtx{
        services: HashMap::new(),
      };
      let app_service = service::AppService{};
      ctx.services.insert("app_service".to_string(), Box::new(Arc::new(app_service)));

      let app_service = ctx.services.get("app_service");
      let app_service = app_service.unwrap();
      let app_service = app_service.clone();
      let app_service = app_service.downcast_ref::<Arc<service::AppService>>().unwrap();
      let app_controller = controller::AppController{
        app_service: app_service.clone(),
      };
      let router = app_controller.register();

      nestrs::DynamicModule{
        router: router,
      }
    }
}

struct ModuleCtx{
  services: HashMap<String, Box<dyn Any>>
}