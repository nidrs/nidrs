pub mod options;
pub mod service;

use nidrs::{DynamicModule, Service};
use nidrs_macro::module;

pub use options::ConfOptions;
use service::ConfService;

use self::options::ConfOptionsProvider;

#[module({
  services = [ConfService];
  exports = [ConfService];
})]
#[derive(Clone, Debug, Default)]
pub struct ConfModule;

impl ConfModule {
    pub fn for_root<T: Into<ConfOptions>>(options: T) -> DynamicModule {
        DynamicModule::new().provider(ConfOptionsProvider::new(options.into()))
    }
}
