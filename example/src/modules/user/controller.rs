use std::collections::HashMap;

use axum::extract::Query;
use nidrs::{AppResult, Inject};
use nidrs_macro::{controller, get};

use super::service::UserService;

#[controller("/user")]
#[derive(Debug, Default)]
pub struct UserController {
    user_service: Inject<UserService>,
}

impl UserController {
    #[get("/hello")]
    pub async fn get_hello_world(&self, Query(q): Query<HashMap<String, String>>) -> AppResult<String> {
        println!("Query {:?}", q);
        Ok(self.user_service.extract().get_hello_world2())
    }
}
