use std::{fmt::Debug, usize::MAX};

use nidrs::macros::interceptor;
use nidrs::{
    externs::axum::{body::to_bytes, extract::FromRequest, http::Response, response::IntoResponse},
    valid::validator::Validator,
};
use nidrs::{AnyResponse, Inject, InterCtx, Interceptor, IntoAnyResponse, StateCtx};

use crate::{AppResult, CurrentUser};

#[interceptor()]
pub struct UserInterceptor {}

impl<B: FromRequest<StateCtx> + Debug + Validator, P: IntoResponse> Interceptor<B, P> for UserInterceptor {
    type R = AnyResponse;

    async fn interceptor<F, H>(&self, ctx: InterCtx<B>, handler: H) -> AppResult<Self::R>
    where
        F: std::future::Future<Output = AppResult<P>> + Send + 'static,
        H: FnOnce(InterCtx<B>) -> F,
    {
        ctx.body.valid()?;

        let r: AppResult<AnyResponse> = handler(ctx).await.map(|r| AnyResponse::from_response(r));

        r
    }
}
