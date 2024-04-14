use axum::{body::Body, http::{HeaderName, HeaderValue, StatusCode}, response::{IntoResponse, IntoResponseParts, Response, ResponseParts}};
use axum_extra::headers::Header;
use nidrs::{Exception, HookCtx, Inject, Interceptor, InterceptorHook};
use nidrs_macro::interceptor;

use crate::{app::dto::Status, AppError, AppResult};

use super::service::LogService;

#[interceptor()]
#[derive(Default)]
pub struct LogInterceptor{
  log_service: Inject<LogService>
}

// impl InterceptorHook for LogInterceptor {
//   async fn before(&self, _ctx: &HookCtx) -> Result<(), Exception> {
//     println!("ctx: {:?}", _ctx.meta);
//     // 获取时间搓
//     self.log_service.log("Before");
//   //  Err((StatusCode::INTERNAL_SERVER_ERROR, "Error".to_string()))
//     Ok(())
//   }

//   async fn after(&self, _ctx: &HookCtx, r: impl IntoResponse) ->Result<String, Exception>{
//     self.log_service.log("After");
//     let resp = r.into_response();
//     let is_suc = resp.status().is_success();
//     let body = resp.into_body();
//     let body_bytes = axum::body::to_bytes(body, usize::MAX).await.unwrap();
//     let body_str = String::from_utf8_lossy(&body_bytes);
//     println!("ctx: {:?}", body_str);
//     self.log_service.log("After");
//     Ok(if is_suc { format!("{{\"code\": 0,\"data\": {}}}", body_str) } else {body_str.to_string()})
//   }  

//   // async fn catch(){

//   // }
// }

impl InterceptorHook for LogInterceptor {
    type P = Status;

    type R = String;

    async fn interceptor<P, F, H>(&self, ctx: HookCtx, handler: H) -> AppResult<Self::R>
    where
        P: Into<Self::P>,
        F: std::future::Future<Output = AppResult<P>> + Send + 'static,
        H: FnOnce(HookCtx) -> F {
        println!("ctx: {:?}", ctx.meta);
        // 获取时间搓
        self.log_service.log("Before");
        //  Err((StatusCode::INTERNAL_SERVER_ERROR, "Error".to_string()))
        let r = handler(ctx).await;

        self.log_service.log("After");
        
        Ok(String::from("Hello, World!"))
    }
}

// pub struct SetHeader<'a>(&'a str, &'a str);

// impl<'a> IntoResponseParts for SetHeader<'a> {
//     type Error = (StatusCode, String);

//     fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
//         // return  Err((
//         //     StatusCode::INTERNAL_SERVER_ERROR,
//         //     format!("Invalid header name {}", self.0),
//         // ));
//         match (self.0.parse::<HeaderName>(), self.1.parse::<HeaderValue>()) {
//             (Ok(name), Ok(value)) => {
//                 res.headers_mut().insert(name, value);
//             },
//             (Err(_), _) => {
//                 return Err((
//                     StatusCode::INTERNAL_SERVER_ERROR,
//                     format!("Invalid header name {}", self.0),
//                 ));
//             },
//             (_, Err(_)) => {
//                 return Err((
//                     StatusCode::INTERNAL_SERVER_ERROR,
//                     format!("Invalid header value {}", self.1),
//                 ));
//             },
//         }

//         Ok(res)
//     }
// }

// impl<'a> IntoResponse for SetHeader<'a> {
//   fn into_response(self) -> Response {
//       // This gives an empty response with the header
//       (self, ()).into_response()
//   }
// }
