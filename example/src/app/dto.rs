use axum::{body::Body, http::{header, StatusCode}, response::{IntoResponse, Response}};
use serde::{Deserialize, Serialize};

pub enum AppError {
    NotFound,
    InternalServerError,
}

pub type AppResult<T> = Result<T, ()>;


#[derive(Serialize, Deserialize, Debug)]
pub struct Status{
    pub code: i32,
    pub message: String,
}

// impl IntoResponse
impl IntoResponse for Status {
    fn into_response(self) -> Response {
        let json_body = match serde_json::to_string(&self) {
        Ok(json) => json,
        Err(_) => return Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body("Internal server error".into())
            .unwrap(),
        };

        // 构建响应，设定状态码和内容类型
        let res: Response<Body> = Response::builder()
            .status(StatusCode::from_u16(self.code as u16).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR))
            .header(header::CONTENT_TYPE, "application/json")
            .body(json_body.into())
            .unwrap();

        res
    }
}