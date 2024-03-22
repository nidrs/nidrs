#[derive(Clone, Debug, Default)]
pub struct AppService{}

impl AppService {
    pub fn get_hello_world(&self) -> String {
        todo!()
    }
}

impl nestrs::Service for AppService {
    
}