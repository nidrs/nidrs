use nidrs::{injectable, module};

#[injectable]
pub struct UserService {}

impl UserService {
    pub async fn get_user(&self) -> String {
        "call get_user".to_string()
    }
}

#[module({
  services: [UserService],
  exports: [UserService]
})]
pub struct UserModule;
