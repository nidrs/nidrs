use nestrs::Inject;


#[derive(Clone, Debug, Default)]
pub struct AppService{
    user_service: Inject<crate::user::service::UserService>
}

impl AppService {
    pub fn get_hello_world(&self) -> String {
        "Hello, Nestrs!".to_string()
    }
}

impl nestrs::Service for AppService {
    
}