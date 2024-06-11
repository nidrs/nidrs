use nidrs_extern::{metadata::ServiceName, once_cell};
use once_cell::sync::OnceCell;
use std::{any::Any, sync::Arc};

use crate::{ImplMeta, ModuleCtx};

pub trait Service: ImplMeta {
    fn inject(&self, ctx: ModuleCtx, module_name: &str) -> ModuleCtx {
        ctx
    }
}

#[derive(Clone, Debug, Default)]
pub struct Inject<T> {
    value: OnceCell<Arc<T>>,
}

impl<T> Inject<T> {
    pub fn new() -> Self {
        Inject { value: OnceCell::new() }
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
        self.value.get().unwrap_or_else(|| panic!("{} not inject.", std::any::type_name::<T>()))
    }
}

pub fn provider<T: Service + 'static>(service: T) -> (&'static str, Box<dyn Any>) {
    let name = T::__meta().get_data::<ServiceName>().unwrap().value();
    (name, Box::new(Arc::new(service)) as Box<dyn Any>)
}
