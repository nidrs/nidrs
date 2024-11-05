use nidrs::macros::{controller, get};
use nidrs::openapi::api;
use nidrs::{post, AppResult, Inject};
use nidrs_extern::axum::Json;
use nidrs_macro::meta;

use super::dto::{WxLoginDto, WxLoginResDto};
// use crate::modules::user::service::UserService;

#[controller("/auth")]
pub struct AuthController {
    // auth_service: Inject<UserService>,
}

impl AuthController {
    #[meta(disable_auto_json = true)]
    #[post("/wxlogin")]
    pub async fn wxlogin(&self, dto: Json<WxLoginDto>) -> AppResult<Json<WxLoginResDto>> {
        // let openid = dto.appid.to_string();
        // let user = self.auth_service.login(openid).await?;
        Ok(Json(WxLoginResDto {
            openid: "oWxxx".to_string(),
        }))
    }
}
