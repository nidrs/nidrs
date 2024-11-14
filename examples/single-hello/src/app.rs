use nidrs::{controller, get, module, AppResult, Module, ModuleCtx};

#[module({
    controllers:[AppController]
})]
pub struct AppModule;

#[controller("/app")]
pub struct AppController{}

impl AppController{
    #[get("/hello")]
    pub async fn get(&self)->AppResult<String>{
        Ok("hello".to_string())
    }
}
