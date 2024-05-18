use std::collections::HashMap;

use axum::extract::Query;
use nidrs::{AppResult, Inject};
use nidrs_macro::{controller, get};

use crate::modules::log::service::LogService;

use super::service::UserService;

#[controller("/user")]
pub struct UserController {
    user_service: Inject<UserService>,
    log_service: Inject<LogService>,
}

impl UserController {
    #[get("/hello")]
    pub async fn get_hello_world(&self, Query(q): Query<HashMap<String, String>>) -> AppResult<String> {
        println!("Query {:?}", q);
        self.log_service.log("hello");
        Ok(self.user_service.extract().get_hello_world2())
    }
}
