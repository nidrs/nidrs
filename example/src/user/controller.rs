use std::collections::HashMap;

use axum::{extract::{Query, State}, Json};
use nidrs::{Inject, StateCtx};
use nidrs_macro::{controller, get, post, meta};

use super::service::UserService;


#[controller("/user")]
#[derive(Debug, Default)]
pub struct UserController {
    user_service: Inject<UserService>,
}

impl UserController {
    #[get("/hello")]
    pub async fn get_hello_world(&self, Query(q): Query<HashMap<String, String>>) -> String {
        println!("Query {:?}", q);
        self.user_service.extract().get_hello_world2()
    }
}
