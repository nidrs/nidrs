#[derive(Debug, serde::Deserialize)]
pub struct CreateUserDto {
    pub name: String,
}
