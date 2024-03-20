use std::{any::Any, collections::HashMap, rc::Rc};

use nestrs_macro::module;

use crate::AppState;

pub mod controller;
pub mod service;

#[module(
  controllers = [controller::AppController],
  services = [service::AppService]
)]
pub struct  AppModule;

impl nestrs::Module for AppModule {
    fn register() -> nestrs::DynamicModule {
      println!("Registering App Module");
      let mut ctx = Ctx{
        services: HashMap::new(),
      };
      let app_service = service::AppService{};
      ctx.services.insert("app_service".to_string(), Box::new(Rc::new(app_service)));

      let app_service = ctx.services.get("app_service");
      let app_service = app_service.unwrap();
      let app_service = app_service.clone();
      let app_service = app_service.downcast_ref::<Rc<service::AppService>>().unwrap();
      let app_controller = controller::AppController{
        app_service: app_service.clone(),
      };

      nestrs::DynamicModule{
        controllers: vec![],
        services: vec![]
      }
    }
}

struct Ctx{
  services: HashMap<String, Box<dyn Any>>
}