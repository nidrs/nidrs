
#[derive(Clone, Debug, Default)]
pub struct AppService{}

impl AppService {
    pub fn get_hello_world(&self) -> String {
        "Hello, Nestrs!".to_string()
    }
}

impl nestrs::Service for AppService {
    
}