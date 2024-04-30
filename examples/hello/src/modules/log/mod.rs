use nidrs::meta;
use nidrs_macro::module;

pub mod interceptor;
pub mod service;

use interceptor::LogInterceptor;
use service::LogService;

#[meta(global)]
#[module({
  interceptors: [LogInterceptor],
  services: [LogService],
  exports: [LogService],
})]
#[derive(Clone, Debug, Default)]
pub struct LogModule;
