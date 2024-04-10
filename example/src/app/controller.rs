use std::{collections::HashMap, sync::Arc};

use axum::{extract::{Query, State}, http::StatusCode, Json};
use nidrs::{Inject, StateCtx};
use nidrs_macro::{controller, get, meta, post, uses};

use super::{dto::{AppResult, Status}, service::AppService};

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
    pub async fn get_hello_world(&self, Query(q): Query<HashMap<String, String>>) -> AppResult<(StatusCode, Status)> {
        println!("Query {:?}", q);
        Ok(( StatusCode::OK,
            Status {
            code: 201,
            message: "Hello, World!".to_string(),
        }))
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
