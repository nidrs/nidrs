use nidrs::Inject;
use nidrs_macro::{injectable, on_module_init};
use crate::user::service::UserService;

#[injectable()]
#[derive(Clone, Debug, Default)]
pub struct AppService{
    user_service: Inject<UserService>
}

impl AppService {
    
    #[on_module_init()]
    pub fn on_module_init(&self){
        let user_service = self.user_service.lock().unwrap();
        let user_service = user_service.as_ref().unwrap();
        println!("AppService on_module_init {}", user_service.get_hello_world());
    }
    
    pub fn get_hello_world(&self) -> String {
        let user_service = self.user_service.lock().unwrap();
        let user_service = user_service.as_ref().unwrap();
        user_service.get_hello_world()
    }

    pub fn get_hello_world2(&self) -> String {
        "Hello, nidrs2xx333!".to_string()
    }
}