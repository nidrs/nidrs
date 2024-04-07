use nidrs::{Inject, Interceptor, HookCtx, InterceptorHook};
use nidrs_macro::interceptor;

use super::service::LogService;

#[interceptor()]
#[derive(Default)]
pub struct LogInterceptor{
  log_service: Inject<LogService>
}

impl InterceptorHook for LogInterceptor {
  async fn before(&self, _ctx: &HookCtx) {
    println!("ctx: {:?}", _ctx.meta);
    // 获取时间搓
    self.log_service.log("Before");
  }

  async fn after(&self, _ctx: &HookCtx) {
    self.log_service.log("After");
  }
}