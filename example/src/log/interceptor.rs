use axum::{body::Body, http::{HeaderName, HeaderValue, StatusCode}, response::{IntoResponse, IntoResponseParts, Response, ResponseParts}, Json};
use axum_extra::headers::Header;
use nidrs::{AnyResponse, Exception, HookCtx, Inject, Interceptor, InterceptorHook, IntoAnyResponse};
use nidrs_macro::interceptor;

use crate::{app::dto::Status, AppError, AppResult};

use super::service::LogService;

#[interceptor()]
#[derive(Default)]
pub struct LogInterceptor{
  log_service: Inject<LogService>
}

impl <P:IntoAnyResponse> InterceptorHook<P> for LogInterceptor {
    type R = AnyResponse;

    async fn interceptor<F, H>(&self, ctx: HookCtx, handler: H) -> AppResult<Self::R>
    where
      F: std::future::Future<Output = AppResult<P>> + Send + 'static,
      H: FnOnce(HookCtx) -> F,
    {
        println!("ctx: {:?}", ctx);
        self.log_service.log("Before");
        let r: AppResult<AnyResponse> = handler(ctx).await.map(|r|IntoAnyResponse::from_serializable(r));

        self.log_service.log("After");
        
        // Ok(Response::builder().body(r.unwrap().body.unwrap()).unwrap())
        r
    }
}