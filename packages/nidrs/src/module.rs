use nidrs_extern::axum;
use nidrs_extern::tokio;
use std::{any::Any, collections::HashMap};

use crate::{provider, AppResult, Service};

pub trait Module {
  fn init(self, ctx: ModuleCtx)->ModuleCtx;
}

pub struct DynamicModule {
  pub services: HashMap<&'static str, Box<dyn Any>>,
}

impl DynamicModule {
  pub fn new() -> Self {
    DynamicModule {
        services: HashMap::new(),
    }
  }

  pub fn service(mut self, service: (&'static str, Box<dyn Any>)) -> Self {
    self.services.insert(service.0, service.1);
    self
  }

  pub fn provider<T:Service + 'static>(mut self, service:T) -> Self {
    let (name, service) = provider(service);
    self.services.insert(name, service);
    self
  }
}

pub struct NidrsFactory<T: Module> {
  pub default_version: &'static str,
  pub default_prefix: &'static str,
  pub module: T,
}

impl <T: Module>NidrsFactory<T> {
  pub fn create(
      module: T,
  ) -> Self {
      NidrsFactory {
        module: module,
        default_prefix: "",
        default_version: "v1",
      }
  }

  pub fn default_prefix(mut self, prefix: &'static str) -> Self {
    self.default_prefix = prefix;
    self
  }

  pub fn default_version(mut self, v: &'static str) -> Self {
    self.default_version = v;
    self
  }

  pub fn listen(self, port: u32) {
    let router = axum::Router::new().route("/", axum::routing::get(|| async move {
      "Hello, Nidrs!"
    }));
    let mut module_ctx = ModuleCtx::new();
    module_ctx.default_prefix = self.default_prefix;
    module_ctx.default_version = self.default_version;
    let module_ctx = self.module.init(module_ctx);
    let routers = module_ctx.routers;
    let mut sub_router = axum::Router::new();
    for router in routers.iter() {
        sub_router = sub_router.merge(router.clone());
    }
    let router = router.merge(sub_router);

    // listen...
    let server = || async {
      let tcp = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
      let addr = tcp.local_addr()?;
      nidrs_macro::log!("Listening on {}", addr);
      
      axum::serve(tcp, router.with_state(StateCtx{})).await?;

      AppResult::Ok(())
    };
    
    let _ = tokio::runtime::Runtime::new().unwrap().block_on(server());
  }
}

#[derive(Debug, Clone)]
pub struct StateCtx{
}

pub struct ModuleCtx{
  pub default_version: &'static str,
  pub default_prefix: &'static str,
  pub modules:HashMap<String, Box<dyn Any>>,
  pub services: HashMap<String, Box<dyn Any>>,
  pub controllers: HashMap<String, Box<dyn Any>>,
  pub routers: Vec<axum::Router<StateCtx>>,
  pub interceptors: HashMap<String, Box<dyn Any>>,
}

impl ModuleCtx {
  pub fn new() -> Self {
      ModuleCtx {
          default_version: "v1",
          default_prefix: "",
          modules: HashMap::new(),
          services: HashMap::new(),
          controllers: HashMap::new(),
          routers: Vec::new(),
          interceptors: HashMap::new(),
      }
  }
}
