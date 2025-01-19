use crate::modules::user::service::UserService;
use nidrs::Inject;
use nidrs_macro::injectable;

#[injectable()]
pub struct AppService {
    user_service: Inject<UserService>,
}

impl AppService {
    pub fn get_hello_world2(&self) -> String {
        "Hello, nidrs2xx333!".to_string()
    }
}
