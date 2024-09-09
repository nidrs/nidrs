use nidrs::openapi::utoipa;
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

#[derive(utoipa::IntoParams)]
#[dto]
pub struct FilterDto {
    pub filter: String,
    pub page: i32,
    pub size: i32,
}
