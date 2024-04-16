use std::fmt::Debug;

use axum::{body::Body, extract::FromRequest, http::{HeaderName, HeaderValue, StatusCode}, response::{IntoResponse, IntoResponseParts, Response, ResponseParts}, Json};
use axum_extra::headers::Header;
use nidrs::{AnyBody, Exception, InterCtx, Inject, InterceptorService, Interceptor, IntoAnyBody, StateCtx};
use nidrs_macro::interceptor;
use serde::{de::DeserializeOwned, Serialize, Serializer};

use crate::{app::dto::Status, AppError, AppResult};

use super::service::LogService;

#[interceptor()]
#[derive(Default)]
pub struct LogInterceptor{
  log_service: Inject<LogService>
}

impl <B: FromRequest<StateCtx> + Debug, P:IntoAnyBody> Interceptor<B, P> for LogInterceptor {
    type R = AnyBody;

    async fn interceptor<F, H>(&self, ctx: InterCtx<B>, handler: H) -> AppResult<Self::R>
    where
      F: std::future::Future<Output = AppResult<P>> + Send + 'static,
      H: FnOnce(InterCtx<B>) -> F,
    {
        println!("ctx: {:?}", ctx);
        self.log_service.log("Before");
        let r: AppResult<AnyBody> = handler(ctx).await.map(|r|IntoAnyBody::from_serializable(r));

        self.log_service.log("After");
        
        // Ok(Response::builder().body(r.unwrap().body.unwrap()).unwrap())
        r
    }
}