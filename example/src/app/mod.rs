#![allow(warnings, unused)]
use std::{any::Any, collections::HashMap, ptr::NonNull, rc::Rc, sync::{Arc, Mutex}};

use axum::Router;
use nestrs::{StateCtx, Inject, Module, Controller, Service};
use nestrs_macro::module;

use crate::AppState;

pub mod controller;
pub mod service;

use controller::AppController;
use service::AppService;
use crate::user::UserModule;

#[module({
    imports = [UserModule];
    controllers = [AppController];
    services = [AppService];
})]
pub struct AppModule;

// impl nestrs::Module for AppModule {
//     fn register(self, ctx: &nestrs::ModuleCtx) -> nestrs::DynamicModule {
//       println!("Registering App Module");
//       let base_router: Router<StateCtx> = axum::Router::new();

//       // let user_module = UserModule::default();
//       // let user_module_dyn = user_module.register(ctx);


//       ctx.services.lock().unwrap().insert("AppService".to_string(), Box::new(Arc::new(service::AppService::default())) as Box<dyn Any>);

//       ctx.controllers.lock().unwrap().insert("AppController".to_string(), Box::new(Arc::new(controller::AppController::default())));

//       let controllers = ctx.controllers.lock().unwrap();

//       let t_controller = controllers.get("AppController").unwrap();
//       let t_controller = t_controller.downcast_ref::<Arc<controller::AppController>>().unwrap();
//       let t_controller = t_controller.clone();
//       ctx.routers.lock().unwrap().push(axum::Router::new().route(
//         "/app/hello",
//         axum::routing::get(|state| async move { // Specify the lifetime of the captured variable
//           t_controller.get_hello_world(state).await
//         }),
//       ));

//       let t_controller = controllers.get("AppController").unwrap();
//       let t_controller = t_controller.downcast_ref::<Arc<controller::AppController>>().unwrap();
//       let t_controller = t_controller.clone();
//       ctx.routers.lock().unwrap().push(axum::Router::new().route(
//         "/app/hello2",
//         axum::routing::get(|state| async move { // Specify the lifetime of the captured variable
//           t_controller.get_hello_world2(state).await
//         }),
//       ));
      

//       let t_controller = controllers.get("AppController").unwrap();
//       let t_controller = t_controller.downcast_ref::<Arc<controller::AppController>>().unwrap();
//       let mut t_controller = t_controller.clone();
//       t_controller.inject(ctx);

      
//       nestrs::DynamicModule{
//       }
//     }
// }
