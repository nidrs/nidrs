use std::{any::Any, collections::HashMap, sync::{Arc, MutexGuard}};

use nestrs::Inject;
use nestrs_macro::injectable;

use crate::app::service::AppService;


#[injectable()]
#[derive(Clone, Debug, Default)]
pub struct UserService{
  pub app_service: Inject<AppService>,
}

impl UserService {
    pub fn get_hello_world(&self) -> String {
      let app_service = self.app_service.lock().expect("Failed to lock app_service");
      println!("Get Hello World {:?}", app_service);
      let app_service = app_service.as_ref().expect("Failed to get app_service");
      app_service.get_hello_world2()
    }
}