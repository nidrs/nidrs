use nidrs::{AppResult, Inject};
use nidrs_macro::injectable;

use crate::models::entities::user::{User, UserEntity};

use super::dto::CreateUserDto;

#[injectable()]
pub struct UserService {
    user_entity: Inject<UserEntity>,
}

impl UserService {
    pub async fn create(&self, part: CreateUserDto) -> AppResult<usize> {
        self.user_entity.create(part.name).await
    }

    pub async fn find_by_id(&self, id: i32) -> AppResult<User> {
        self.user_entity.find_by_id(id).await
    }

    pub async fn all(&self) -> AppResult<Vec<User>> {
        self.user_entity.all().await
    }

    pub async fn update_name_by_id(&self, id: i32, name: String) -> AppResult<usize> {
        self.user_entity.update(id, name).await
    }
}
