use std::collections::HashMap;

use nidrs::{externs::axum::extract::Query, meta, post};
use nidrs::{
    macros::{controller, get},
    uses,
};
use nidrs::{AppResult, Inject};
use nidrs_extern::axum::Json;

// use crate::modules::log::service::LogService;

use super::{dto::CreateUserDto, service::UserService};

#[controller("/user")]
pub struct UserController {
    user_service: Inject<UserService>,
    // log_service: Inject<LogService>,
}

impl UserController {
    // #[uses(UserInterceptor)]
    #[get("/hello")]
    pub async fn get_hello_world(&self, Query(q): Query<HashMap<String, String>>) -> AppResult<String> {
        println!("Query {:?}", q);
        // self.log_service.log("hello");
        Ok(self.user_service.extract().get_hello_world2())
    }

    // #[meta(nidrs::datasets::RouterBodyScheme::from_type::<CreateUserDto>())]
    // #[uses(UserInterceptor)]
    #[post("/")]
    pub async fn create_user(&self, dto: Json<CreateUserDto>) -> AppResult<String> {
        Ok(self.user_service.extract().get_hello_world2())
    }
}
