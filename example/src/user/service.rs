use std::{any::Any, collections::HashMap, sync::{Arc, MutexGuard}};

use nestrs::Inject;


#[derive(Clone, Debug, Default)]
pub struct UserService{
  pub app_service: Inject<crate::app::service::AppService>,
}

impl UserService {
    pub fn get_hello_world(&self) -> String {
      let app_service = self.app_service.lock().expect("Failed to lock app_service");
      println!("Get Hello World {:?}", app_service);
      let app_service = app_service.as_ref().expect("Failed to get app_service");
      app_service.get_hello_world2()
    }
}

impl nestrs::Service for UserService {
    fn inject(&self, services: &MutexGuard<HashMap<String, Box<dyn Any>>>) {
      let app_service = services.get("AppService");
      if let Some(app_service) = app_service {
        let app_service = app_service.downcast_ref::<Arc<crate::app::service::AppService>>().unwrap();
        self.app_service.inject(app_service.clone());
      } 
    }
}