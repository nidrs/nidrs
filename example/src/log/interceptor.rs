use axum::{http::{HeaderName, HeaderValue, StatusCode}, response::{IntoResponse, IntoResponseParts, Response, ResponseParts}};
use axum_extra::headers::Header;
use nidrs::{Inject, Interceptor, HookCtx, InterceptorHook};
use nidrs_macro::interceptor;

use crate::AppResult;

use super::service::LogService;

#[interceptor()]
#[derive(Default)]
pub struct LogInterceptor{
  log_service: Inject<LogService>
}

impl InterceptorHook for LogInterceptor {
  type R = (SetHeader<'static>, String);
  type E = (StatusCode, String);

  async fn before(&self, _ctx: &HookCtx) -> Result<(), Self::E> {
    println!("ctx: {:?}", _ctx.meta);
    // 获取时间搓
    self.log_service.log("Before");
  //  Err((StatusCode::INTERNAL_SERVER_ERROR, "Error".to_string()))
    Ok(())
  }

  async fn after(&self, _ctx: &HookCtx, r: impl IntoResponse) ->Result<(SetHeader<'static> , String), Self::E>{
    self.log_service.log("After");
    let body = r.into_response().into_body();
    let body_bytes = axum::body::to_bytes(body, usize::MAX).await.unwrap();
    let body_str = String::from_utf8_lossy(&body_bytes);
    println!("ctx: {:?}", body_str);
    self.log_service.log("After");
    Ok((SetHeader("content-type", "application/json"), format!("{{\"code\": 0,\"data\": {}}}", body_str)))
  }  
}


pub struct SetHeader<'a>(&'a str, &'a str);

impl<'a> IntoResponseParts for SetHeader<'a> {
    type Error = (StatusCode, String);

    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        // return  Err((
        //     StatusCode::INTERNAL_SERVER_ERROR,
        //     format!("Invalid header name {}", self.0),
        // ));
        match (self.0.parse::<HeaderName>(), self.1.parse::<HeaderValue>()) {
            (Ok(name), Ok(value)) => {
                res.headers_mut().insert(name, value);
            },
            (Err(_), _) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Invalid header name {}", self.0),
                ));
            },
            (_, Err(_)) => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Invalid header value {}", self.1),
                ));
            },
        }

        Ok(res)
    }
}

impl<'a> IntoResponse for SetHeader<'a> {
  fn into_response(self) -> Response {
      // This gives an empty response with the header
      (self, ()).into_response()
  }
}
