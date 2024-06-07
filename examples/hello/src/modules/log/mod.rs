use nidrs::macros::module;
use nidrs::meta;
use nidrs::metadata::Global;

pub mod interceptor;
pub mod service;

use interceptor::LogInterceptor;
use service::LogService;

#[meta(Global::Enabled)]
#[module({
  interceptors: [LogInterceptor],
  services: [LogService],
  exports: [LogService],
})]
pub struct LogModule;
