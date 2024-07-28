use std::fmt::Debug;

use axum::extract::FromRequest;
use nidrs::{AnyBody, InterCtx, InterceptorHandler, IntoAnyBody, StateCtx};
use nidrs_macro::interceptor;

use crate::AppResult;

#[interceptor()]
pub struct JsonInterceptor {}

impl<B: FromRequest<StateCtx> + Debug, P: IntoAnyBody> InterceptorHandler<B, P> for JsonInterceptor {
    type R = AnyBody;

    async fn interceptor<F, H>(&self, ctx: InterCtx<B>, handler: H) -> AppResult<Self::R>
    where
        F: std::future::Future<Output = AppResult<P>> + Send + 'static,
        H: FnOnce(InterCtx<B>) -> F,
    {
        let r: AppResult<AnyBody> = handler(ctx).await.map(|r| IntoAnyBody::from_serializable(r));

        r
    }
}
