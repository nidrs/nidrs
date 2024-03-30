use std::sync::{Arc, Mutex};

use nidrs::Inject;
use nidrs_macro::injectable;

use crate::app::service::AppService;


#[injectable()]
#[derive(Clone, Debug, Default)]
pub struct UserService{
  app_service: Inject<AppService>,
  count: Arc<Mutex<i32>>
}

impl UserService {
    pub fn get_hello_world(&self) -> String {
      let app_service = self.app_service.lock().expect("Failed to lock app_service");
      println!("Get Hello World {:?}", self.count);
      let app_service = app_service.as_ref().expect("Failed to get app_service");
      app_service.get_hello_world2()
    }

    pub fn get_hello_world2(&self) -> String {
      let mut count = self.count.lock().unwrap();
      *count += 1;
      format!("Hello, World! {}", count)
    }
}