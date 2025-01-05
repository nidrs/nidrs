use nidrs::openapi::utoipa;

#[nidrs::openapi::schema]
#[nidrs::valid::dto]
pub struct CreateUserDto {
    // #[rule(Email, "age must be greater than 0")]
    pub name: String,

    // #[rule(Number::default().max(12).min(0))]
    pub age: i32,
}

#[nidrs::openapi::schema]
#[nidrs::valid::dto]
pub struct UserByIdDto {
    pub id: i32,
}

#[nidrs::openapi::schema]
#[nidrs::valid::dto]
pub struct FilterDto {
    pub id: i32,
    pub filter: String,
    pub page: i32,
    pub size: i32,
}

#[nidrs::openapi::schema]
#[nidrs::valid::dto]
pub struct CreateUserResDto {
    pub id: i32,
    pub name: String,
}
