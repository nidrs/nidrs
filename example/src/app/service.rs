use nestrs::Inject;
use nestrs_macro::injectable;
use crate::user::service::UserService;

#[injectable()]
#[derive(Clone, Debug, Default)]
pub struct AppService{
    user_service: Inject<UserService>
}

impl AppService {
    pub fn get_hello_world(&self) -> String {
        let user_service = self.user_service.lock().unwrap();
        let user_service = user_service.as_ref().unwrap();
        user_service.get_hello_world()
    }

    pub fn get_hello_world2(&self) -> String {
        "Hello, Nestrs2xx333!".to_string()
    }
}