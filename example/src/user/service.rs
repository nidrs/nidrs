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
}