use serde::{Deserialize, Serialize};

use crate::Validator;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserDto {
    pub name: String,
    pub age: i32,
}

impl Validator for CreateUserDto {
    fn valid(&self) -> bool {
        self.age > 0
    }
}
