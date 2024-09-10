use nidrs::openapi::utoipa;
use nidrs::valid_macro::dto;

#[nidrs::openapi::schema]
#[dto]
pub struct CreateUserDto {
    #[rule(Email, "age must be greater than 0")]
    pub name: String,

    #[rule(Number::default().max(12).min(0))]
    pub age: i32,
}

#[nidrs::openapi::schema]
#[dto]
pub struct UserByIdDto {
    pub id: i32,
}

#[nidrs::openapi::schema]
#[dto]
pub struct FilterDto {
    pub filter: String,
    pub page: i32,
    pub size: i32,
}
