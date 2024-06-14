use std::collections::HashMap;

use nidrs::{externs::axum::extract::Query, post, throw};
use nidrs::{
    macros::{controller, get},
    Exception,
};
use nidrs::{AppResult, Inject};
use nidrs_extern::{
    anyhow,
    axum::{http::StatusCode, Json},
};

use crate::{modules::log::service::LogService, Valid};

use super::{dto::CreateUserDto, service::UserService};

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
    #[post("/")]
    pub async fn create_user(&self, dto: Json<CreateUserDto>) -> AppResult<String> {
        let c = Valid::from(&dto).check()?;
        println!("c {:?}", c);
        Ok(self.user_service.extract().get_hello_world2())
    }
}
