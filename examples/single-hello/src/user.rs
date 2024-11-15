use nidrs::{controller, get, injectable, module, AppResult, Module, ModuleCtx};


#[injectable]
pub struct UserService{}

impl UserService {
  pub async fn get_user(&self)->String{
    return "call get_user".to_string();
  }
}

#[module({
  services: [UserService],
  exports: [UserService]
})]
pub struct UserModule;

