use std::{collections::HashSet, rc::Rc};

use nestrs_macro::{controller, get};

use super::{service, Ctx};
use std::sync::Arc;

#[controller("/app")]
pub struct  AppController{
  pub app_service: Arc<service::AppService>
}

impl AppController {
  #[get("/hello-world")]
  fn get_hello_world(&self)-> String{
    self.app_service.get_hello_world()
  }

}

impl AppController {
    pub fn register(&self){
      let app_service = Arc::clone(&self.app_service);
      let get_hello_world = || async move {
        app_service.get_hello_world();
        "Hello, World!".to_string()
      };
      let route1 = axum::Router::<String>::new().route("/api/v1/user/register", axum::routing::get(get_hello_world));
    }
}

impl nestrs::Controller for AppController {
}