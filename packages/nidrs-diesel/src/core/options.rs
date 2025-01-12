use nidrs::{ImplMeta, Service};

use super::drivers::driver::ConnectionDriver;

#[derive(Default)]
pub struct DieselOptions<D: Into<ConnectionDriver> = ConnectionDriver> {
    pub driver: D,
}

impl Service for DieselOptions {}

impl ImplMeta for DieselOptions {
    fn __meta(&self) -> nidrs::InnerMeta {
        let mut meta = nidrs::InnerMeta::new();
        meta.set("service_name", "DieselOptions");
        meta
    }
}
