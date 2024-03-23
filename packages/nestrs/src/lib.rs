#![allow(warnings, unused)]


use std::{any::Any, fmt::Debug, sync::Arc};
pub trait Module {
    fn register(self) -> DynamicModule;
}

pub trait Controller {}

pub trait Service {}

pub struct DynamicModule {
    pub router: axum::Router<Ctx>,
}

pub struct NestFactory {
    router: axum::Router<Ctx>,
}

impl NestFactory {
    pub fn create<T: Module, S>(
        module: T,
        state: S,
    ) -> Self {
        let router = axum::Router::new().route("/", axum::routing::get(|| async move {
            "Hello, World!"
        }));
        let dynamic_module = module.register();
        NestFactory {
            router: router.merge(dynamic_module.router)
        }
    }

    pub async fn listen<E>(self, port: u32) -> Result<(), E>
    where
        E: std::convert::From<std::io::Error>,
    {
        let tcp = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.map_err(E::from)?;
        let addr = tcp.local_addr().map_err(E::from)?;
        println!("Listening on {}", addr);
        
        axum::serve(tcp, self.router.with_state(Ctx{})).await?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Ctx{
}

pub type Inject<T> = Arc<T>;
