use nidrs_extern::{
    ruleset,
    validator::{Rule, ValidResult},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::Validator;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserDto {
    // #[rule(ruleset::Email, "age must be greater than 0")]
    pub name: String,
    pub age: i32,
}

impl Validator for CreateUserDto {
    fn valid(&self) -> ValidResult {
        ruleset::Email::default().set_message("name not email").valid(&self.name)?;

        return Ok(());
    }
}

// CreateUserDto::mock_data();
