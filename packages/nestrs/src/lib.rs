use std::any::Any;

pub trait Module {
  fn register() -> DynamicModule;
}

pub trait Controller {
    
}

pub trait  Service {
    
}

pub struct  DynamicModule{
    pub controllers: Vec<Box<dyn Controller>>,
    pub services: Vec<Box<dyn Service>>
}

pub struct NestFactory{}

impl NestFactory {
    pub fn create<T: Module>(module: T, app_state: Box<dyn Any>) -> NestFactory {
        let dynamic_module = T::register();
        NestFactory{}
    }

    pub fn listen(&self, port: u32) {
        println!("Listening on port {}", port);
    }
}
