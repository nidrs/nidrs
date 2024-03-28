use std::sync::Arc;

use nestrs::Inject;


#[derive(Clone, Debug, Default)]
pub struct AppService{
    user_service: Inject<crate::user::service::UserService>
}

impl AppService {
    pub fn get_hello_world(&self) -> String {
        let user_service = self.user_service.lock().unwrap();
        let user_service = user_service.as_ref().unwrap();
        user_service.get_hello_world()
    }

    pub fn get_hello_world2(&self) -> String {
        "Hello, Nestrs2xx!".to_string()
    }
}

impl nestrs::Service for AppService {
    fn inject(&self, ctx: &nestrs::ModuleCtx) {
        let binding = ctx.services.clone();
        let binding = binding.lock().unwrap();
        let user_service = binding.get("UserService").unwrap();
        let user_service = user_service.downcast_ref::<Arc<crate::user::service::UserService>>().unwrap();
        self.user_service.inject(user_service.clone().into());
    }
}