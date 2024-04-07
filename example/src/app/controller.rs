use std::{collections::HashMap, sync::Arc};

use axum::{extract::{Query, State}, Json};
use nidrs::{Inject, StateCtx};
use nidrs_macro::{controller, get, meta, post, uses};

use super::service::AppService;

#[meta(role = "admin", auth = "true")]
#[uses(LogInterceptor)]
#[controller("/app")]
#[derive(Debug, Default)]
pub struct AppController {
    app_service: Inject<AppService>,
}

impl AppController {
    #[get("/hello")]
    #[meta(role = "user")]
    #[uses(LogInterceptor)]
    pub async fn get_hello_world(&self, Query(q): Query<HashMap<String, String>>) -> String {
        println!("Query {:?}", q);
        self.app_service.get_hello_world()
    }

    #[get("/hello2")]
    #[uses(LogInterceptor)]
    pub async fn get_hello_world2(&self, Query(q): Query<HashMap<String, String>>) -> String {
        println!("Query {:?}", q);
        self.app_service.get_hello_world()
    }

    #[post("/hello")]
    pub async fn post_hello_world(&self, Query(q): Query<HashMap<String, String>>, Json(j): Json<serde_json::Value>) -> String {
        println!("Query {:?}", q);
        println!("Json {:?}", j);

        "Hello, World2!".to_string()
    }
}
