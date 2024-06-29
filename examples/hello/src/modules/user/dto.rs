use nidrs::valid_macro::dto;
use nidrs_extern::utoipa;

#[derive(utoipa::ToSchema)]
#[dto]
pub struct CreateUserDto {
    #[rule(Email, "age must be greater than 0")]
    pub name: String,

    #[rule(Number::default().max(12).min(0))]
    pub age: i32,
}
