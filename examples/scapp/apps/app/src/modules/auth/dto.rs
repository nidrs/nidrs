use nidrs::openapi::schema;
use serde::{Deserialize, Serialize};

#[schema]
#[derive(Debug, Serialize, Deserialize)]
pub struct WxLoginDto {
    pub appid: String,
    pub code: String,
}

#[schema]
#[derive(Debug, Serialize, Deserialize)]
pub struct WxLoginResDto {
    pub openid: String,
}
