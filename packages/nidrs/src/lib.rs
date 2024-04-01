#![allow(warnings, unused)]


use std::{any::Any, cell::RefCell, collections::HashMap, fmt::Debug, sync::{Arc, Mutex, MutexGuard}};
pub trait Module {
    fn init(self, ctx: &ModuleCtx);
}


pub trait Service {
    fn inject(&self, services: &MutexGuard<HashMap<String, Box<dyn Any>>>);
}

pub trait Controller {
    fn inject(&self, services: &MutexGuard<HashMap<String, Box<dyn Any>>>);
}

pub struct DynamicModule {
    pub services: HashMap<String, Option<Box<dyn Any>>>,
}

pub struct NidrsFactory {
    pub router: axum::Router<StateCtx>,
}

impl NidrsFactory {
    pub fn create<T: Module>(
        module: T,
    ) -> Self {
        let router = axum::Router::new().route("/", axum::routing::get(|| async move {
            "Hello, Nidrs!"
        }));
        let module_ctx = ModuleCtx::new();
        module.init(&module_ctx);
        let routers = module_ctx.routers.lock().unwrap();
        let mut sub_router = axum::Router::new();
        for router in routers.iter() {
            sub_router = sub_router.merge(router.clone());
        }
        NidrsFactory {
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
    
    pub fn extract(&self) -> Arc<T> {
        self.value.lock().unwrap().as_ref().unwrap().clone()
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
    pub modules: Arc<Mutex<HashMap<String, Box<dyn Any>>>>,
    pub services: Arc<Mutex<HashMap<String, Box<dyn Any>>>>,
    pub controllers: Arc<Mutex<HashMap<String, Box<dyn Any>>>>,
    pub routers: Arc<Mutex<Vec<axum::Router<StateCtx>>>>
}

impl ModuleCtx {
    pub fn new() -> Self {
        ModuleCtx {
            modules: Arc::new(Mutex::new(HashMap::new())),
            services: Arc::new(Mutex::new(HashMap::new())),
            controllers: Arc::new(Mutex::new(HashMap::new())),
            routers: Arc::new(Mutex::new(Vec::new())),
        }
    }
}


#[cfg(test)]
mod tests {
    use std::{any::Any, sync::Mutex};
    use crate::*;

    struct ConfOptions {
        log_level: String,
    }

    fn get_dynamic_module() -> DynamicModule {
        DynamicModule {
            services: HashMap::from([("ConfOptions".to_string(), Some(Box::new(Arc::new(ConfOptions{
                log_level: "info".to_string(),
            })) as Box<dyn Any>))])
        }
    }

    #[test]
    fn it_take(){
        let module_ctx = ModuleCtx::new();
        let dynamic_module = get_dynamic_module();
        let mut services = dynamic_module.services;
        for (key, value) in services.iter_mut() {
            let t = value.take();
            module_ctx.services.lock().unwrap().insert(key.clone(), t.unwrap());
        }
        println!("{:?}", module_ctx.services.lock().unwrap());
    }
}

 