#![allow(warnings, unused)]


use std::{any::Any, cell::RefCell, collections::HashMap, fmt::Debug, sync::{Arc, Mutex, MutexGuard}};
pub trait Module {
    fn register(self, ctx: &ModuleCtx) -> DynamicModule;
}


pub trait Service {
    fn inject(&self, services: &MutexGuard<HashMap<String, Box<dyn Any>>>);
}

pub trait Controller {
    fn inject(&self, services: &MutexGuard<HashMap<String, Box<dyn Any>>>);
}

pub struct DynamicModule {

}

pub struct NestFactory {
    pub router: axum::Router<StateCtx>,
}

impl NestFactory {
    pub fn create<T: Module, S>(
        module: T,
        state: S,
    ) -> Self {
        let router = axum::Router::new().route("/", axum::routing::get(|| async move {
            "Hello, World!"
        }));
        let module_ctx = ModuleCtx::new();
        let dynamic_module = module.register(&module_ctx);
        let routers = module_ctx.routers.lock().unwrap();
        let mut sub_router = axum::Router::new();
        for router in routers.iter() {
            // 打印路由信息, path 和 method
            sub_router = sub_router.merge(router.clone());
        }
        NestFactory {
            router: router.merge(sub_router),
        }
    }

    pub async fn listen<E>(self, port: u32) -> Result<(), E>
    where
        E: std::convert::From<std::io::Error>,
    {
        let tcp = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.map_err(E::from)?;
        let addr = tcp.local_addr().map_err(E::from)?;
        println!("Listening on {}", addr);
        
        axum::serve(tcp, self.router.with_state(StateCtx{})).await?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct StateCtx{
}

#[derive(Clone, Debug, Default)]
pub struct Inject<T>{
    value: Arc<Mutex<Option<Arc<T>>>>
}

impl<T> Inject<T> {
    pub fn new() -> Self {
        Inject {
            value: Arc::new(Mutex::new(None))
        }
    }
    
    pub fn inject(&self, value: Arc<T>) {
        self.value.lock().unwrap().replace(value);
    }
    
 
}

impl<T> std::ops::Deref for Inject<T> {
    type Target = Mutex<Option<Arc<T>>>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[derive(Debug, Clone)]
pub struct ModuleCtx{
    pub services: Arc<Mutex<HashMap<String, Box<dyn Any>>>>,
    pub controllers: Arc<Mutex<HashMap<String, Box<dyn Any>>>>,
    pub routers: Arc<Mutex<Vec<axum::Router<StateCtx>>>>
}

impl ModuleCtx {
    pub fn new() -> Self {
        ModuleCtx {
            services: Arc::new(Mutex::new(HashMap::new())),
            controllers: Arc::new(Mutex::new(HashMap::new())),
            // router: Arc::new(Mutex::new(axum::Router::new())),
            routers: Arc::new(Mutex::new(Vec::new())),
        }
    }
}


#[cfg(test)]
mod tests {
    use std::{any::Any, sync::Mutex};


    trait Service {
        fn inject(&self);
    }

    struct UserService {
    }

    impl Service for UserService {
        fn inject(&self) {
            println!("Inject UserService");
        }
    }

    struct  Injectable{
        value: Box<dyn Any>
    }

    impl Injectable {
        fn new(value: Box<dyn Any>) -> Self {
            Injectable {
                value
            }
        }
    }
    

    #[test]
    fn it_works() {
        let map = Mutex::new(std::collections::HashMap::<String, Box<dyn Service>>::new());
        // let mut map1 = map.lock().unwrap();
        // let mut map2 = map.lock().unwrap();
        // map1.insert("UserService".to_string(), Box::new(UserService{}));
        // map1.insert("UserService".to_string(), Box::new(UserService{}));
        map.lock().unwrap().insert("UserService".to_string(), Box::new(UserService{}));
        map.lock().unwrap().insert("UserService".to_string(), Box::new(UserService{}));

        // let user_service = map1.get("UserService").unwrap();
        // user_service.inject();
    }
}

 