#![allow(warnings, unused)]


use std::{any::Any, collections::HashMap, fmt::Debug, sync::{Arc, Mutex}};
pub trait Module {
    fn register(self, ctx: &ModuleCtx) -> DynamicModule;
}


pub trait Service {
}

pub trait Controller {
    fn register(self) -> axum::Router<StateCtx>;
}

pub struct DynamicModule {

}

pub struct NestFactory {
    router: axum::Router<StateCtx>,
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

#[derive(Debug, Clone, Default)]
pub struct Inject<T>{
    value: Option<Arc<T>>
}

impl<T> Inject<T> {
    pub fn new(value: T) -> Self {
        Inject {
            value: Some(Arc::new(value))
        }
    }
    
    pub fn inject(&self, services: Arc<HashMap<String, Box<dyn Any>>>) {
        let value = self.value.clone();
    }
    
 
}

impl<T> std::ops::Deref for Inject<T> {
    type Target = Arc<T>;

    fn deref(&self) -> &Self::Target {
        &self.value.as_ref().unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct ModuleCtx{
    pub services: Arc<Mutex<HashMap<String, Box<dyn Any>>>>,
    // pub controllers: Arc<Mutex<HashMap<String, Arc<Mutex<dyn Any>>>>>,
    pub controllers: Arc<Mutex<HashMap<String, Box<dyn Any>>>>,
    // pub router: Arc<Mutex<axum::Router<StateCtx>>>
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