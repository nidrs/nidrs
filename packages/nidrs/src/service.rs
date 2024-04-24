use nidrs_extern::once_cell;
use once_cell::sync::OnceCell;
use std::any::Any;

use crate::{ImplMeta, ModuleCtx};

pub trait Service: Any {
    fn inject(&self, ctx: ModuleCtx) -> ModuleCtx;

    fn as_any(&self) -> &dyn Any;
}

#[derive(Clone, Debug, Default)]
pub struct Inject<T: Clone> {
    value: OnceCell<T>,
}

impl<T: Clone> Inject<T> {
    pub fn new() -> Self {
        Inject { value: OnceCell::new() }
    }

    pub fn inject(&self, value: T) {
        let _ = self.value.set(value);
    }

    pub fn extract(&self) -> T {
        self.value.get().unwrap().clone()
    }
}

impl<T: Clone> std::ops::Deref for Inject<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.value.get().unwrap()
    }
}

pub fn provider<T: Service + ImplMeta + 'static>(service: T) -> (&'static str, Box<dyn Service>) {
    let name = *T::__meta().get::<&str>("service_name").unwrap();
    (name, Box::new(service))
}
