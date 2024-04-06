use nidrs_macro::module;

pub mod service;
pub mod interceptor;

use service::LogService;
use interceptor::LogInterceptor;

#[module({
  services = [LogService];
  exports = [LogService];
})]
#[derive(Clone, Debug, Default)]
pub struct LogModule;