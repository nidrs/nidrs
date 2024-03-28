use std::sync::Arc;

use nestrs::Inject;


#[derive(Clone, Debug, Default)]
pub struct UserService{
  pub app_service: Inject<crate::app::service::AppService>,
}

impl UserService {
    pub fn get_hello_world(&self) -> String {
      // self.app_service.unwrap().clone().get_hello_world();
        "Hello, Nestrs!".to_string()
    }
}

impl nestrs::Service for UserService {
    fn inject(&self, ctx: &nestrs::ModuleCtx) {
      let binding = ctx.services.clone();
      let binding = binding.lock().unwrap();
      let app_service = binding.get("AppService").unwrap();
      let app_service = app_service.downcast_ref::<Arc<crate::app::service::AppService>>().unwrap();
      self.app_service.inject(app_service.clone());
    }
}