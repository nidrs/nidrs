#![allow(warnings, unused)]

use nidrs_extern::axum::{self, async_trait, extract::{FromRequestParts, Query, State}, http::{request::Parts, HeaderMap, HeaderValue, StatusCode, Uri}, response::IntoResponse};
use nidrs_extern::tokio;
use once_cell::sync::OnceCell;
use std::{any::Any, cell::{RefCell}, collections::HashMap, fmt::Debug, sync::{Arc, Mutex, MutexGuard}};

pub trait Module {
    fn init(self, ctx: &ModuleCtx);
}


pub trait Service {
    fn inject(&self, services: &MutexGuard<HashMap<String, Box<dyn Any>>>);
}

pub trait Interceptor {
    fn inject(&self, services: &MutexGuard<HashMap<String, Box<dyn Any>>>);
}

pub trait InterceptorHook {
    async fn before(&self, ctx: &HookCtx);
    async fn after(&self, ctx: &HookCtx);
}

pub trait Controller {
    fn inject(&self, services: &MutexGuard<HashMap<String, Box<dyn Any>>>);
}

#[derive(Debug, Clone)]
pub struct InterReq{
    pub uri: Uri,
    pub method: axum::http::Method,
    pub headers: HeaderMap<HeaderValue>,
    pub query: HashMap<String, String>,
}

#[async_trait]
impl<S> FromRequestParts<S> for InterReq
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        return  Ok(InterReq{
            uri: parts.uri.clone(),
            method: parts.method.clone(),
            headers: parts.headers.clone(),
            query: HashMap::new(),
        });
    }
}


pub struct DynamicModule {
    pub services: HashMap<String, Option<Box<dyn Any>>>,
}

#[derive(Debug, Clone)]
pub struct HookCtx{
    pub meta: HashMap<String, String>,
    pub req: InterReq,
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
        nidrs_macro::log!("Listening on {}", addr);
        
        axum::serve(tcp, self.router.with_state(StateCtx{})).await?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct StateCtx{
}

#[derive(Clone, Debug, Default)]
pub struct Inject<T>{
    value: OnceCell<Arc<T>>
}

impl<T> Inject<T> {
    pub fn new() -> Self {
        Inject {
            value: OnceCell::new()
        }
    }
    
    pub fn inject(&self, value: Arc<T>) {
        self.value.set(value);
    }
    
    pub fn extract(&self) -> Arc<T> {
        self.value.get().unwrap().clone()
    }
}

impl<T> std::ops::Deref for Inject<T> {
    type Target = Arc<T>;
    fn deref(&self) -> &Self::Target {
        self.value.get().unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct ModuleCtx{
    pub modules: Arc<Mutex<HashMap<String, Box<dyn Any>>>>,
    pub services: Arc<Mutex<HashMap<String, Box<dyn Any>>>>,
    pub controllers: Arc<Mutex<HashMap<String, Box<dyn Any>>>>,
    pub routers: Arc<Mutex<Vec<axum::Router<StateCtx>>>>,
    pub interceptors: Arc<Mutex<HashMap<String, Box<dyn Any>>>>,
}

impl ModuleCtx {
    pub fn new() -> Self {
        ModuleCtx {
            modules: Arc::new(Mutex::new(HashMap::new())),
            services: Arc::new(Mutex::new(HashMap::new())),
            controllers: Arc::new(Mutex::new(HashMap::new())),
            routers: Arc::new(Mutex::new(Vec::new())),
            interceptors: Arc::new(Mutex::new(HashMap::new())),
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

    #[test]
    fn it_map(){
        let map1 = HashMap::from([("a".to_string(), 1), ("b".to_string(), 2)]);
        let mut map2 = HashMap::from([("a".to_string(), 2), ("b".to_string(), 2)]);
        map2.extend(map1);
        let map1 = map2;

        println!("{:?}", map1);
    }
}

 