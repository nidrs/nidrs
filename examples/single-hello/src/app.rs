use nidrs::{controller, get, module, AppResult, Module, ModuleCtx};


#[controller("/app")]
pub struct AppController{}

impl AppController{
    #[get("/hello")]
    pub async fn get(&self)->AppResult<String>{
        Ok("hello".to_string())
    }
}

#[module({
    controllers:[AppController]
})]
pub struct AppModule;

