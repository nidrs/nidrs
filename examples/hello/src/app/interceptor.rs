use nidrs::{injectable, AppResult, Interceptor};
use nidrs_extern::axum::{extract::Request, middleware::Next, response::IntoResponse};

#[injectable]
pub struct AppInterceptor;

impl Interceptor for AppInterceptor {
    async fn intercept(&self, req: Request, next: Next) -> AppResult<impl IntoResponse> {
        println!("Intercepting request: {:?}", req);
        let res = next.run(req).await;
        println!("Intercepting response: {:?}", res);

        Ok(res)
    }
}
