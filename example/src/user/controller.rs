use std::collections::HashMap;

use axum::{extract::{Query, State}, Json};
use nidrs::{Inject, StateCtx};
use nidrs_macro::{controller, get, post};

use super::service::UserService;


#[controller("/user")]
#[derive(Debug, Default)]
pub struct UserController {
    user_service: Inject<UserService>,
}

impl UserController {
    #[get("/hello")]
    pub async fn get_hello_world(&self, State(state): State<StateCtx>, Query(q): Query<HashMap<String, String>>) -> String {
        println!("Query {:?}", q);
        let user_service = self.user_service.lock().unwrap();
        let user_service = user_service.as_ref().unwrap();
        user_service.get_hello_world2()
    }
}
