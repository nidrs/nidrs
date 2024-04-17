use nidrs_extern::axum::{body::{Body, Bytes}, response::Response};
use nidrs_extern::axum::{self, http::StatusCode, response::IntoResponse};
use std::{collections::HashMap, fmt::Debug, future::Future};
use serde::Serialize;

use crate::{AppError, AppResult, ModuleCtx, ServiceProperty, StateCtx};

pub trait InterceptorService {
  fn inject(&self, ctx: ModuleCtx) -> ModuleCtx;
  fn property() -> ServiceProperty;
}

/// P 和 R 是可以配置的
pub trait Interceptor<B: axum::extract::FromRequest<StateCtx>, P>: Sized {
  type R;

  fn interceptor<F, H>(&self, ctx: InterCtx<B>, handler: H) -> impl Future<Output = AppResult<Self::R>>
  where
      F: std::future::Future<Output = AppResult<P>> + Send + 'static,
      H: FnOnce(InterCtx<B>) -> F;
}


#[derive(Debug)]
pub struct AnyBody<T = ()>{
  pub body: Result<Bytes, AppError>,
  marker: std::marker::PhantomData<T>,
}

impl Serialize for AnyBody {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
      S: serde::Serializer,
  {
      self.body.as_ref().map(|b| b.as_ref()).unwrap().serialize(serializer)
  }
  
}

impl IntoResponse for AnyBody {
  fn into_response(self) -> Response {
      let body = match self.body {
          Ok(b) => b,
          Err(e) => Bytes::from(e.to_string()),
      };

      Response::builder()
          .status(StatusCode::OK)
          .header("Content-Type", "application/json")
          .body(Body::from(body))
          .unwrap()
  }
}

pub trait IntoAnyBody: Sized + serde::Serialize {
  fn from_serializable<T: serde::Serialize>(s: T) -> AnyBody<Self>;
}

impl<T: serde::Serialize> IntoAnyBody for T {
  fn from_serializable<P: serde::Serialize>(s: P) -> AnyBody<Self> {
      AnyBody {
          body: serde_json::to_vec(&s).map(Bytes::from).map_err(|e| e.into()),
          marker: std::marker::PhantomData,
      }
  }
}

#[derive(Debug)]
pub struct InterCtx<B: axum::extract::FromRequest<StateCtx>>{
    pub meta: HashMap<String, String>,
    pub parts: axum::http::request::Parts,
    pub body: B,
}