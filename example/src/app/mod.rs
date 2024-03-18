use crate::AppState;

pub mod controller;
pub mod service;

// #[module(
//   controllers = [controller::AppController],
//   services = [service::AppService]
// )]
pub struct  AppModule;

impl nestrs::Module for AppModule {
    fn register() -> nestrs::DynamicModule {
      println!("Registering App Module");
      let app_service = service::AppService{};
      let app_controller = controller::AppController::new(app_service);
      // let ctx = Ctx{
      //   app_service
      // };
      nestrs::DynamicModule{
        controllers: vec![Box::new(controller::AppController::new(service::AppService{}))],
        services: vec![Box::new(service::AppService{})]
      }
    }
}

struct Ctx{
  app_service: service::AppService
}