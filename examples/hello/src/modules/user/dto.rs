use nidrs_extern::{
    ruleset,
    validator::{Rule, ValidResult},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::Validator;

// #[dto]
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserDto {
    // #[rule(ruleset::Email, "age must be greater than 0")]
    pub name: String,
    // #[rule(ruleset::Number{ max: 20, min: 10 }, "age must be greater than 0")]
    // #[rule(expr!(v > 10 && v < 100), "age must be greater than 0")]
    pub age: i32,
}

impl Validator for CreateUserDto {
    fn valid(&self) -> ValidResult {
        use ruleset::*;
        ruleset::Email.valid(&self.name, Some("'name' is not an Email"))?;
        ruleset::Number.valid(&self.name, Some("'name' is not an Email"))?;
        ruleset::Number.valid(self.age, Some("'name' is not an Email"))?;
        // expr!(self.age > 10 && self.age < 100, "age must be greater than 0")?;
        return Ok(());
    }
}

// CreateUserDto::mock_data();
