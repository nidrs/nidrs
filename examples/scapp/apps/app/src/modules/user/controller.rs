use nidrs::macros::{controller, get};
use nidrs::openapi::api;
use nidrs::{post, AppResult, Inject};
use nidrs_extern::axum::Json;
use nidrs_macro::meta;

use crate::models::dao::users::{CreateUser, User};

use super::dto::LoginDto;
use super::service::UserService;

#[controller("/user")]
pub struct UserController {
    user_service: Inject<UserService>,
}

impl UserController {
    #[api]
    #[post("/register")]
    pub async fn register(&self, dto: Json<LoginDto>) -> AppResult<Json<User>> {
        let user = self.user_service.login(dto.openid.to_owned()).await?;
        return Ok(Json(user));
    }
}
