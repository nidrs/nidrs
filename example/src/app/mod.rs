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
      // let ctx = Ctx{
      //   app_service
      // };
      nestrs::DynamicModule{
        controllers: vec![],
        services: vec![]
      }
    }
}

struct Ctx{
  app_service: service::AppService
}