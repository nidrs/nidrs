use nidrs::{ImplMeta, Service};

use super::drivers::driver::ConnectionDriver;

#[derive(Default)]
pub struct DieselOptions<D: Into<ConnectionDriver> = ConnectionDriver> {
    pub driver: D,
    pub name: Option<String>,
}

impl<D: Into<ConnectionDriver>> DieselOptions<D> {
    pub fn new(driver: D) -> Self {
        Self { driver, name: None }
    }

    pub fn with_name<N: Into<String>>(mut self, name: N) -> Self {
        self.name = Some(name.into());
        self
    }
}

impl Service for DieselOptions {}

impl ImplMeta for DieselOptions {
    fn __meta(&self) -> nidrs::InnerMeta {
        let mut meta = nidrs::InnerMeta::new();
        meta.set("service_name", "DieselOptions");
        meta
    }
}
