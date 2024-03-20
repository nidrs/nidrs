use nestrs_macro::{controller, get};

use super::service;

#[controller("/app")]
pub struct  AppController{
  app_service: service::AppService
}

impl AppController {
  #[get("/hello-world")]
  fn get_hello_world(&self)-> String{
    self.app_service.get_hello_world()
  }
}

impl nestrs::Controller for AppController {
}