use nidrs_extern::*;
use nidrs_extern::tokio;
use std::{any::Any, collections::HashMap, sync::{Arc, Mutex}};

pub trait Module {
  fn init(self, ctx: &ModuleCtx);
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
      nidrs_macro::log!("Listening on {}", addr);
      
      axum::serve(tcp, self.router.with_state(StateCtx{})).await?;
      Ok(())
  }
}

#[derive(Debug, Clone)]
pub struct StateCtx{
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
