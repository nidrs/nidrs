use nidrs::externs::serde_json;
use nidrs::openapi::utoipa;
use nidrs::{
    externs::axum::{
        body::Body,
        http::{header, StatusCode},
        response::{IntoResponse, Response},
    },
    valid::dto,
};
use utoipa::ToSchema;

// #[derive(nidrs::openapi::utoipa::ToSchema)]
#[nidrs::openapi::schema]
#[dto]
pub struct Status {
    pub db: String,
    pub redis: String,
    pub mongo: Mongo,
}

impl IntoResponse for Status {
    fn into_response(self) -> Response {
        let json_body = match serde_json::to_string(&self) {
            Ok(json) => json,
            Err(_) => return Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR).body("Internal server error".into()).unwrap(),
        };

        // 构建响应，设定状态码和内容类型
        let res: Response<Body> = Response::builder()
            // .status(StatusCode::from_u16(self.code as u16).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR))
            .header(header::CONTENT_TYPE, "application/json")
            .body(json_body.into())
            .unwrap();

        res
    }
}

// impl nidrs::openapi::ToParamDto for Status {
//     fn to_param_dto(
//         dto_type: nidrs::openapi::ParamDtoIn,
//     ) -> nidrs::openapi::ParamDto {
//         use nidrs::openapi::utoipa::IntoParams;
//         use nidrs::openapi::utoipa::ToSchema;
//         use nidrs::openapi::utoipa::openapi::Schema;
//         use nidrs::openapi::utoipa::openapi::RefOr;
//         use nidrs::openapi::utoipa;

//         let schema: RefOr<Schema> = utoipa::schema!(Self).into();

//         match dto_type {
//             nidrs::openapi::ParamDtoIn::Body => {
//                 nidrs::openapi::ParamDto::BodySchema((
//                     schema,
//                     vec![],
//                 ))
//             }
//             _ => nidrs::openapi::ParamDto::None
//         }
//     }
// }

#[nidrs::openapi::schema]
#[dto]
pub struct Mongo {
    pub count: u64,
}

#[derive(ToSchema)]
#[dto]
pub struct A {
    // #[rule(Email)]
    pub hello: String,

    // #[rule(Valid(v))]
    pub hello2: B,
}

#[derive(ToSchema)]
#[dto]
pub struct B {
    pub hello2: String,
}

#[derive(ToSchema)]
#[dto]
pub enum ArgDto {
    A(A),
    B(B),
}

#[dto]
pub struct ArgWrapDto(pub ArgDto);
