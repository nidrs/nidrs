use nidrs_extern::once_cell;
use once_cell::sync::OnceCell;
use std::{any::Any, sync::Arc};

use crate::{Meta, ModuleCtx};

pub trait Service {
    fn inject(&self, ctx: ModuleCtx) -> ModuleCtx;
    fn property() -> ServiceProperty;
}


pub struct ServiceProperty{
    pub name: &'static str,
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
        let _ = self.value.set(value);
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


pub fn provider<T:Service + 'static>(service: T) -> (&'static str, Box<dyn Any>){
    (T::property().name, Box::new(Arc::new(service)) as Box<dyn Any>)
}