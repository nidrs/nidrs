use nidrs::externs::serde;
use nidrs::openapi::schema;

use serde::Deserialize;

#[schema]
#[derive(Debug, Deserialize)]
pub struct CreateUserDto {
    pub name: String,
}
