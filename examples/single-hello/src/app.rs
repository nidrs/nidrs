use nidrs::{controller, get, module, AppResult, Inject, Module, ModuleCtx};
use crate::user::{UserModule, UserService};


#[controller("/app")]
pub struct AppController{
    user_service: Inject<UserService>
}

impl AppController{
    #[get("/hello")]
    pub async fn get(&self)->AppResult<String>{
        Ok(self.user_service.get_user().await)
    }
}

#[module({
    imports: [UserModule],
    controllers:[AppController]
})]
pub struct AppModule;

