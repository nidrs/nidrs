use nidrs::openapi::schema;
use serde::{Deserialize, Serialize};

#[schema]
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginDto {
    pub openid: String,
}
