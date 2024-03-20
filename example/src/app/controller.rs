use std::rc::Rc;

use nestrs_macro::{controller, get};

use super::{service, Ctx};

#[controller("/app")]
pub struct  AppController{
  pub app_service: Rc<service::AppService>
}

impl AppController {
  #[get("/hello-world")]
  fn get_hello_world(&self)-> String{
    self.app_service.get_hello_world()
  }

}

impl nestrs::Controller for AppController {
}