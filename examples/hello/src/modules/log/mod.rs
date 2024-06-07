use nidrs::{global, macros::module};

pub mod interceptor;
pub mod service;

use interceptor::LogInterceptor;
use service::LogService;

#[global]
#[module({
  interceptors: [LogInterceptor],
  services: [LogService],
  exports: [LogService],
})]
pub struct LogModule;
