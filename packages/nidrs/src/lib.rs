#![allow(warnings, unused)]

use nidrs_extern::{anyhow::anyhow, axum::{body::{Body, Bytes}, extract::Request, response::Response}, tower::Layer, *};
use nidrs_extern::axum::{self, async_trait, extract::{FromRequestParts, Query, State}, http::{request::Parts, HeaderMap, HeaderValue, StatusCode, Uri}, response::IntoResponse};
use nidrs_extern::tokio;
use once_cell::sync::OnceCell;
use std::{any::Any, cell::RefCell, collections::HashMap, error::Error, fmt::Debug, sync::{Arc, Mutex, MutexGuard}, task::{Context, Poll}};

pub use nidrs_macro::*;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Environment variable not found")]
    EnvironmentVariableNotFound(#[from] std::env::VarError),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    
    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),

    #[error(transparent)]
    Exception(#[from] Exception),
}
impl IntoResponse for AppError{
    fn into_response(self) -> axum::response::Response {
        axum::response::Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(format!("Error: {}", self.to_string()))
            .unwrap()
        .into_response()
    }
}

pub type AppResult<T = ()> = Result<T, AppError>;

pub trait Module {
    fn init(self, ctx: &ModuleCtx);
}


pub trait Service {
    fn inject(&self, services: &MutexGuard<HashMap<String, Box<dyn Any>>>);
}

pub trait Interceptor {
    fn inject(&self, services: &MutexGuard<HashMap<String, Box<dyn Any>>>);
}


// pub trait InterceptorHook {
//     async fn before(&self, ctx: &HookCtx) -> Result<(), Exception> {
//         Ok(())
//     }
//     async fn after(&self, ctx: &HookCtx, r: impl IntoResponse)-> Result<impl IntoResponse, Exception> {
//         Ok(r)
//     }
// }

pub trait InterceptorHook {
    type P;
    type R;

    async fn interceptor<P, F, H>(&self, ctx: HookCtx, handler: H) -> AppResult<Self::R>
    where
        P: Into<Self::P>,
        F: std::future::Future<Output = AppResult<P>> + Send + 'static,
        H: FnOnce(HookCtx) -> F;
}


pub struct AnyResponse{
    pub body: Result<Bytes, AppError>,
}

impl IntoResponse for AnyResponse {
    fn into_response(self) -> Response {
        let body = match self.body {
            Ok(b) => b,
            Err(e) => Bytes::from(e.to_string()),
        };

        Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(Body::from(body))
            .unwrap()
    }
}


pub trait Controller {
    fn inject(&self, services: &MutexGuard<HashMap<String, Box<dyn Any>>>);
}


pub struct DynamicModule {
    pub services: HashMap<String, Option<Box<dyn Any>>>,
}


#[derive(Debug)]
pub struct HookCtx{
    pub meta: HashMap<String, String>,
    // pub headers: HeaderMap<HeaderValue>,
    // pub body: Bytes,
    // pub request: &'a Request
    // pub request: Request<T>,
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

#[derive(thiserror::Error, Debug)]
pub struct Exception {
    pub status: StatusCode,
    pub error: anyhow::Error,
    pub line: String,
}

impl Exception {
    pub fn new(status: StatusCode, error: anyhow::Error) -> Self {
        Exception {
            status,
            error,
            line: String::new(),
        }
    }
}

impl std::fmt::Display for Exception {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HTTP Exception: status={}, error={}\n   {}", self.status, self.error, self.line)
    }
}

impl IntoResponse for Exception{
    fn into_response(self) -> axum::response::Response {
        axum::response::Html("Internal Server Error".to_string())
        .into_response()
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

 