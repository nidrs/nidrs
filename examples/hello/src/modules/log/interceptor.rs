use std::{fmt::Debug, usize::MAX};

use axum::{body::to_bytes, extract::FromRequest, http::Response, response::IntoResponse};
use nidrs::{AnyResponse, Inject, InterCtx, Interceptor, IntoAnyResponse, StateCtx};
use nidrs_macro::interceptor;

use crate::AppResult;

use super::service::LogService;

#[interceptor()]
pub struct LogInterceptor {
    log_service: Inject<LogService>,
}

impl<B: FromRequest<StateCtx> + Debug, P: IntoResponse> Interceptor<B, P> for LogInterceptor {
    type R = AnyResponse;

    async fn interceptor<F, H>(&self, ctx: InterCtx<B>, handler: H) -> AppResult<Self::R>
    where
        F: std::future::Future<Output = AppResult<P>> + Send + 'static,
        H: FnOnce(InterCtx<B>) -> F,
    {
        println!("ctx: {:?}", ctx.meta.get::<bool>("disable_default_prefix"));
        self.log_service.log("Before");
        let r: AppResult<AnyResponse> = handler(ctx).await.map(|r| AnyResponse::from_response(r));

        let r = r.unwrap();

        let (parts, body) = r.response.into_parts();

        let body = to_bytes(body, MAX).await.unwrap();

        println!("body: {:?}", body);

        let body = format!("{{\"data\":{}}}", String::from_utf8_lossy(&body));

        let response = Response::from_parts(parts, body.into());

        // body to string

        self.log_service.log("After");

        // Ok(Response::builder().body(r.unwrap().body.unwrap()).unwrap())
        Ok(response.into())
    }
}
