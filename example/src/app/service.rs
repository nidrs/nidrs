use nestrs::Inject;


#[derive(Clone, Debug, Default)]
pub struct AppService{
    user_service: Inject<crate::user::service::UserService>
}

impl AppService {
    pub fn get_hello_world(&self) -> String {
        "Hello, Nestrs!".to_string()
    }

    pub fn get_hello_world2(&self) -> String {
        self.user_service.get_hello_world()
    }
}

impl nestrs::Service for AppService {
    
}