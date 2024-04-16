use nidrs_extern::axum;
use nidrs_extern::tokio;
use std::{any::Any, collections::HashMap};
use crate::{ControllerService, InterceptorService, Service};

pub trait Module {
  fn init(self, ctx: ModuleCtx)->ModuleCtx;
}

pub struct DynamicModule {
  pub services: HashMap<&'static str, Box<dyn Any>>,
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
      let module_ctx = module.init(module_ctx);
      let routers = module_ctx.routers;
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

pub struct ModuleCtx{
  pub modules:HashMap<String, Box<dyn Any>>,
  pub services: HashMap<String, Box<dyn Any>>,
  pub controllers: HashMap<String, Box<dyn Any>>,
  pub routers: Vec<axum::Router<StateCtx>>,
  pub interceptors: HashMap<String, Box<dyn Any>>,
}

impl ModuleCtx {
  pub fn new() -> Self {
      ModuleCtx {
          modules: HashMap::new(),
          services: HashMap::new(),
          controllers: HashMap::new(),
          routers: Vec::new(),
          interceptors: HashMap::new(),
      }
  }
}
