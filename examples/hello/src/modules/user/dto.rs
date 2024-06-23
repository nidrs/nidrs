use nidrs::{
    valid::validator::{Rule, ValidResult},
    valid_macro::Validator,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Validator)]
pub struct CreateUserDto {
    #[rule(Email, "age must be greater than 0")]
    pub name: String,
    // #[rule(Number::default().max(12).min(0), "age must be greater than 0")]
    // #[rule(expr!(v > 10 && v < 100), "age must be greater than 0")]
    pub age: i32,
}

// impl Validator for CreateUserDto {
//     fn valid(&self) -> ValidResult {
//         use ruleset::*;
//         ruleset::Email::default().valid(&self.name, "name", None)?;
//         ruleset::Number::default().max(12).min(10).valid(&self.age, "age", None)?;
//         // expr!(self.age > 10 && self.age < 100, "age must be greater than 0")?;
//         return Ok(());
//     }
// }

// CreateUserDto::mock_data();
