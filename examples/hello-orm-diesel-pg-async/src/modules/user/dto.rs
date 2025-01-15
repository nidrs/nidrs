use nidrs::externs::serde;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateUserDto {
    pub name: String,
}
