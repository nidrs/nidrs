use nidrs::openapi::utoipa::openapi::path::Parameter;
use nidrs::openapi::{utoipa, ParamType, ToParamDto};
use nidrs::valid_macro::dto;

#[derive(utoipa::ToSchema)]
#[dto]
pub struct CreateUserDto {
    #[rule(Email, "age must be greater than 0")]
    pub name: String,

    #[rule(Number::default().max(12).min(0))]
    pub age: i32,
}

#[derive(utoipa::IntoParams)]
#[dto]
pub struct UserByIdDto {
    pub id: i32,
}

#[derive(utoipa::IntoParams, utoipa::ToSchema)]
// #[nidrs::openapi::IntoParams]
#[dto]
pub struct FilterDto {
    pub filter: String,
    pub page: i32,
    pub size: i32,
}

impl ToParamDto for FilterDto {
    fn to_param_dto(dto_type: nidrs::openapi::ParamDtoType) -> nidrs::openapi::ParamDto {
        use nidrs::openapi::utoipa::IntoParams;
        use nidrs::openapi::utoipa::ToSchema;
        match dto_type {
            nidrs::openapi::ParamDtoType::Parameter(p) => nidrs::openapi::ParamDto::Parameters(Self::into_params(|| Some(p.clone()))),
            nidrs::openapi::ParamDtoType::RequestBody => nidrs::openapi::ParamDto::RequestBodies(Self::schema()),
        }
    }
}
