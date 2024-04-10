use axum::response::IntoResponse;
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

  async fn after<T:IntoResponse>(&self, _ctx: &HookCtx, r: T) ->T{
    self.log_service.log("After");
    r
  }
}