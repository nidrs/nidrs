use std::sync::{Arc, Mutex};

use nidrs::macros::injectable;
use nidrs::{AppResult, Inject};

use crate::app::service::AppService;
use crate::models::dao::users::{CreateUser, User, UserEntity};

#[injectable()]
pub struct UserService {
    user_entity: Inject<UserEntity>,
}

impl UserService {
    pub async fn login(&self, openid: String) -> AppResult<User> {
        let res = self.user_entity.create(openid.to_string()).await?;
        let user = self.user_entity.find_by_openid(openid).await?;
        return Ok(user);
    }
}
