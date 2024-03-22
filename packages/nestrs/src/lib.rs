#![allow(warnings, unused)]


use std::{any::Any, fmt::Debug, sync::Arc};
pub trait Module {
    fn register(self, router: axum::Router<Arc<Ctx>>) -> DynamicModule;
}

pub trait Controller {}

pub trait Service {}

pub struct DynamicModule {
    pub controllers: Vec<Box<dyn Controller>>,
    pub services: Vec<Box<dyn Service>>,
}

pub struct NestFactory {
    router: axum::Router<Arc<Ctx>>,
}

impl NestFactory {
    pub fn create<T: Module, S>(
        module: T,
        state: S,
    ) -> Self {
        let router: axum::Router<Arc<Ctx>> = axum::Router::new();
        let dynamic_module = module.register(router.clone());
        NestFactory {
            router: router.with_state(Arc::new(Ctx{}) as Arc<Ctx>),
        }
    }

    pub async fn listen<E>(&self, port: u32) -> Result<(), E>
    where
        E: std::convert::From<std::io::Error>,
    {
        let tcp = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.map_err(E::from)?;
        let addr = tcp.local_addr().map_err(E::from)?;
        println!("Listening on {}", addr);
        let base_router = axum::Router::new();
        let router = self.router.clone();
        base_router.merge(router);
        axum::serve(tcp, base_router).await?;
        Ok(())
    }
}

pub struct Ctx{
}