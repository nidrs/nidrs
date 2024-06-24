use nidrs::valid_macro::dto;

#[dto]
pub struct CreateUserDto {
    #[rule(Email, "age must be greater than 0")]
    pub name: String,

    #[rule(Number::default().max(12).min(0))]
    pub age: i32,
}