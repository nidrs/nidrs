use std::{cell::OnceCell, collections::HashMap, sync::Arc};

pub trait Creator {
    fn create() -> Self;
}

pub trait Svc {
  fn register(&self, ctx: &ModuleCtx);
}

pub trait DynModule {}

pub trait Module {
    fn init(&self, ctx: ModuleCtx) -> ModuleCtx;
    fn destroy(&self, ctx: &ModuleCtx);
}

pub trait Controller:Svc {
  
}
pub trait Service:Svc {}

pub struct ModuleCtx {
  modules: HashMap<String, Box<dyn Module>>,
  controllers: HashMap<String, Box<dyn Controller>>,
  services: HashMap<String, Box<dyn Service>>,
}

impl ModuleCtx {
  pub fn create() -> Self {
      Self {
          modules: HashMap::new(),
          controllers: HashMap::new(),
          services: HashMap::new(),
      }
  }
  pub fn register_module(&mut self, name: &str, module: Box<dyn Module>) {
      self.modules.insert(name.to_string(), module);
  }
  
  pub fn register_router(&self, router: axum::Router) {
  }

  pub fn register_controller<T: Controller>(&self, module_name: &str) {
    // self.controllers.insert(name.to_string(), controller);
    todo!()
  }
  
  pub fn get_svc<T: Svc>(&self) -> Provider<T> {
    // self.services.get(T::name()).unwrap()
    todo!()
  }
}

pub struct NidrsFactory<T: Module> {
  module: T,
  ctx: ModuleCtx,
}

impl<T: Module> NidrsFactory<T> {
  pub fn listen(&self, port: u16) {
  }
  pub fn destroy(self) {
    self.module.destroy(&self.ctx);
  }
}

impl<T: Module + Creator> Creator for NidrsFactory<T> {
  fn create() -> NidrsFactory<T> {
    let module = T::create();
    let ctx = ModuleCtx::create();
    let ctx = module.init(ctx);
    NidrsFactory::<T> { module, ctx }
  }
}


#[derive(Clone, Debug)]
pub struct Provider<T: Svc> {
  svc: OnceCell<Arc<T>>,
}


impl<T: Svc> Provider<T> {
  pub fn new() -> Self {
    Provider { svc: OnceCell::new() }
  }

  pub fn inject(&self, svc: Arc<T>) {
      let _ = self.svc.set(svc);
  }

  pub fn extract(&self) -> Arc<T> {
      self.svc.get().unwrap().clone()
  }
}

impl<T: Svc> std::ops::Deref for Provider<T> {
  type Target = Arc<T>;
  fn deref(&self) -> &Self::Target {
      self.svc.get().unwrap_or_else(|| panic!("{} not inject.", std::any::type_name::<T>()))
  }
}


pub type Inject<T> = OnceCell<Arc<T>>;