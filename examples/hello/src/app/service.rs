use crate::modules::user::service::UserService;
use nidrs::macros::injectable;
use nidrs::Inject;

#[injectable()]
pub struct AppService {
    user_service: Inject<UserService>,
}

impl AppService {
    pub fn get_hello_world(&self) -> String {
        self.user_service.extract().get_hello_world()
    }

    pub fn get_hello_world2(&self) -> String {
        "Hello, nidrs2xx333!".to_string()
    }
}
